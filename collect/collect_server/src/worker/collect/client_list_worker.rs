use std::error::Error;

use dbs;
use dbs::pg_pool::PgUploader;
use dbs_cmd;

use utils::parsing::redis_res::parsing_client_list;

pub fn client_list_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::ClientList).unwrap();
    let result = redis_conn.run_command(cmd, &[])?;
    
    let list = parsing_client_list(result)?;
    let mut client_iter = list.iter();

    let mut t = pg_conn.trans()?;
    let pg_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::ClientList).unwrap();
    
    loop {
        let seq = client_iter.next();
        if seq.is_none() {
            let _ = t.commit();
            break;
        }

        let client = seq.unwrap();
        let exec_ret =  t.execute(&pg_query, &[&link_key, 
            &client.id, &client.addr, &client.fd, &client.name, &client.age,
            &client.idle, &String::from(client.flags), &client.db, &client.sub,
            &client.psub, &(client.multi as i64), &client.qbuf, &client.qbuf_free,
            &client.obl, &client.oll, &client.omem, &String::from(client.events), &client.cmd, &client.user]);

        if exec_ret.is_err() {
            let _ = t.rollback();
            return Err(exec_ret.unwrap_err());
        }
    }

    Ok(())
}