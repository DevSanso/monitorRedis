use std::error::Error;

use dbs;
use dbs_cmd;
use dbs::pg_pool::PgUploader;

use utils::parsing::redis_res::parsing_cluster_nodes;

pub fn cluster_nodes_worker(link_key : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, pg_conn : &'_ mut dbs::pg_pool::PgConnecter) -> Result<(),Box<dyn Error>> {
    let cmd = dbs_cmd::REIDS_COMMANDLINE_MAP.get(&dbs_cmd::RedisCommand::GetClusterNodes).unwrap();
 
    let result = redis_conn.run_command(cmd, &[])?;
    let nodes = parsing_cluster_nodes(result)?;
    

    let mut trans = pg_conn.trans()?;

    let sync_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::SyncClusterNodes).unwrap();
    let push_query = dbs_cmd::PG_COMMANDLINE_MAP.get(&dbs_cmd::PgCommand::InsertClusterNodesPing).unwrap();

    for node in nodes {
        let sync_ret = trans.execute(&sync_query, &[&link_key, &node.node_id.as_str(), 
            &node.port, &node.cport, &node.node_type.as_str(), &node.master_node, &node.ping_epoch, &node.connect_type.as_str()]);

        if sync_ret.is_err() {
            let __ = trans.rollback();
            return Err(sync_ret.err().unwrap());
        }

        let push_ret = trans.execute(&push_query, &[&link_key, &node.node_id.as_str(), &node.ping_send, &node.ping_recv]);

        if push_ret.is_err() {
            let _ = trans.rollback();
            return Err(push_ret.err().unwrap());
        }
    }

    let _ = trans.commit();

    Ok(())
}