use std::error::Error;

use dbs;
use dbs_cmd;

use parsing::redis_res::parsing_client_list;

pub fn client_list_worker(link_key : u32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgUploader) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::ClientList).unwrap();
    let result = redis_conn.run_command(cmd, &[])?;
    
    let list = parsing_client_list(result)?;

    let pg_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::ClientList).unwrap();
    
    for client in list {
        pg_conn.execute(&pg_query, &[&link_key, 
            &(client.id as u32), &client.addr, &(client.fd as u32), &client.name, &(client.age as u32),
            &(client.idle as u32), &String::from(client.flags), &(client.db as u32), &(client.sub as u32),
            &(client.psub as u32), &client.multi, &(client.qbuf as u32), &(client.qbuf_free as u32),
            &(client.obl as u32), &(client.oll as u32), &(client.omem as u32), &String::from(client.events), &client.cmd])?;
    }

    Ok(())
}