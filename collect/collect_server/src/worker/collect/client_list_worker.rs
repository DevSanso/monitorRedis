use std::error::Error;

use dbs;
use dbs_cmd;

use parsing::redis_res::parsing_client_list;

pub fn client_list_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgUploader) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::ClientList).unwrap();
    let result = redis_conn.run_command(cmd, &[])?;
    
    let list = parsing_client_list(result)?;

    let pg_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::ClientList).unwrap();
    
    for client in list {
        pg_conn.execute(&pg_query, &[&link_key, 
            &client.id, &client.addr, &client.fd, &client.name, &client.age,
            &client.idle, &String::from(client.flags), &client.db, &client.sub,
            &client.psub, &(client.multi as i64), &client.qbuf, &client.qbuf_free,
            &client.obl, &client.oll, &client.omem, &String::from(client.events), &client.cmd])?;
    }

    Ok(())
}