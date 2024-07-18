use core::utils_inherit_error;
use std::error::Error;

use dbs;
use dbs_cmd;
use dbs::pg_pool::PgUploader;

use utils::parsing::redis_res::parsing_cluster_nodes;

fn slot_pg_bind_value(arr : &[i32]) -> Option<String> {
    if arr.len() == 0 {
        return None;
    }
    
    let mut str = arr.iter().fold(String::from(""), |mut acc, x| {
        acc.push_str(format!("{},", x).as_str());
        acc
    });

    str.pop();
    Some(str)
}

pub fn cluster_nodes_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::GetClusterNodes).unwrap();
 
    let result = redis_conn.run_command(cmd, &[])?;
    let nodes = parsing_cluster_nodes(result)?;
    
    let sync_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::SyncClusterNodes).unwrap();
    let push_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::InsertClusterNodesPing).unwrap();

    for node in nodes {
        let mut trans = pg_conn.trans()?;

        let sync_ret = trans.execute(&sync_query, &[&link_key, &node.node_id.as_str(), &node.ip.as_str(),
            &node.port, &node.cport, &node.node_type.as_str(), &node.master_node, &node.ping_epoch, &node.connect_type.as_str(), &slot_pg_bind_value(node.slots.as_slice())]);

        if sync_ret.is_err() {
            let __ = trans.rollback();
            return utils_inherit_error!(connection, CommandRunError, "sync", sync_ret.err().unwrap());
        }

        let push_ret = trans.execute(&push_query, &[&link_key, &node.node_id.as_str(), &node.ping_send, &node.ping_recv]);

        if push_ret.is_err() {
            let _ = trans.rollback();
            return utils_inherit_error!(connection, CommandRunError, "push", push_ret.err().unwrap());
        }

        let _ = trans.commit();
    }

    Ok(())
}