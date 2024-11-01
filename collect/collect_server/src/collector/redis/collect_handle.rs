use std::error::Error;

use dbs::utils::make_pg_numeric;
use crate::global::get_redis_global;
use core::utils_inherit_error;
use dbs::pg_pool::PgUploader;
use dbs_cmd::{COLLECT_COMMANDLINE_MAP, CollectCommand};

use crate::utils::parsing::redis_res::*;
use crate::utils::parsing::redis_res::KeyMemUsage;


pub fn client_list_handle(server_id : i32, val : String) -> Result<(), Box<dyn Error>> {
    let list = parsing_client_list(val)?;
    let mut client_iter = list.iter();

    let mut collect_conn_item = {
        get_redis_global().pools.collect_pool.get_owned(())?
    };

    let collect_conn = collect_conn_item.get_value();


    let mut t = collect_conn.trans()?;
    let pg_query = dbs_cmd::COLLECT_COMMANDLINE_MAP.get(&CollectCommand::RedisClientList).unwrap();
    
    loop {
        let seq = client_iter.next();
        if seq.is_none() {
            let _ = t.commit();
            break;
        }

        let client = seq.unwrap();
        let exec_ret =  t.execute(&pg_query, &[&server_id,
            &client.id, &client.addr, &client.fd, &client.name, &client.age,
            &client.idle, &String::from(client.flags), &client.db, &client.sub,
            &client.psub, &(client.multi as i64), &client.qbuf, &client.qbuf_free,
            &client.obl, &client.oll, &client.omem, &String::from(client.events), &client.cmd, &client.user]);

        if exec_ret.is_err() {
            let _ = t.rollback();
            return Err(exec_ret.unwrap_err());
        }
    }
    collect_conn_item.restoration();

    Ok(())
}

#[inline]
fn push_and_sort_buffer_data(dst : &mut [KeyMemUsage;4500], src : &'_ [KeyMemUsage]) {
    dst[1000..src.len() + 1000].clone_from_slice(src);
    dst.sort_by(|a,b| {a.mem_size.partial_cmp(&b.mem_size).unwrap()});
    dst.reverse();
}

pub fn key_usage_top_ten_hundred_handle(server_id : i32, redis_conn : &'_ mut dbs::redis_pool::RedisRequester, cmd :&'_ str) -> Result<(),Box<dyn Error>> {
    use std::thread;
    use std::array;
    use std::time;

    let mut collect_conn_item = {
        get_redis_global().pools.collect_pool.get_owned(())?
    };

    let collect_conn = collect_conn_item.get_value();
    
    let mut cursor = -1;
    let mut buffer : [KeyMemUsage;4500] = array::from_fn(|_| KeyMemUsage::default());

    while cursor != 0 {
        let result_opt = if cursor == -1 {
            Some(redis_conn.run_command(cmd, &[&"0"])?)
        }else {
            let cur = cursor.to_string();
            Some(redis_conn.run_command(cmd, &[&cur.as_str()])?)
        };

        let result = result_opt.unwrap();

        let buf_keyusage = parsing_key_usage_top_ten_hundred(result)?;

        cursor = buf_keyusage.next_cursor;
        push_and_sort_buffer_data(&mut buffer, buf_keyusage.fetch.as_slice());
        thread::sleep(time::Duration::from_millis(1000));
    }

    let mut t = collect_conn.trans()?;
    let pg_query = dbs_cmd::COLLECT_COMMANDLINE_MAP.get(&CollectCommand::RedisInsertKeyUsageTopTenHundred).unwrap();

    for item in buffer.iter().take(1000) {
        let ret = t.execute(pg_query, &[&server_id, &item.name, &item.mem_size, &item.remain_expired_time]);

        if ret.is_err() {
            let _ = t.rollback();
            return Err(ret.err().unwrap());
        }
    }
    let _ = t.commit();

    collect_conn_item.restoration();

    Ok(())
}

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

pub fn cluster_nodes_handle(server_id : i32, val : String) -> Result<(),Box<dyn Error>> {
    let nodes = parsing_cluster_nodes(val)?;
    
    let sync_query = COLLECT_COMMANDLINE_MAP.get(&CollectCommand::RedisSyncClusterNodes).unwrap();
    let push_query = COLLECT_COMMANDLINE_MAP.get(&CollectCommand::RedisInsertClusterNodesPing).unwrap();

    let mut collect_conn_item = {
        get_redis_global().pools.collect_pool.get_owned(())?
    };

    let collect_conn = collect_conn_item.get_value();

    for node in nodes {
        let mut trans = collect_conn.trans()?;

        let sync_ret = trans.execute(&sync_query, &[&server_id, &node.node_id.as_str(), &node.ip.as_str(),
            &node.port, &node.cport, &node.node_type.as_str(), &node.master_node, &node.ping_epoch, &node.connect_type.as_str(), &slot_pg_bind_value(node.slots.as_slice())]);

        if sync_ret.is_err() {
            let __ = trans.rollback();
            return utils_inherit_error!(connection, CommandRunError, "sync", sync_ret.err().unwrap());
        }

        let push_ret = trans.execute(&push_query, &[&server_id, &node.node_id.as_str(), &node.ping_send, &node.ping_recv]);

        if push_ret.is_err() {
            let _ = trans.rollback();
            return utils_inherit_error!(connection, CommandRunError, "push", push_ret.err().unwrap());
        }

        let _ = trans.commit();
    }

    Ok(())
}

pub fn info_cpu_handle(server_id : i32, val : String) -> Result<(),Box<dyn Error>> {
    use log::debug;

    static mut INFO_CPU_OLD : InfoCpu = InfoCpu {
        child_cpu_sys : 0.0,
        child_cpu_user : 0.0,
        cpu_sys : 0.0,
        cpu_user : 0.0,
        total_sec : 0
    };

    let c = parsing_info_cpu(val)?;
    let mut input = InfoCpu::default();
    
    unsafe {
        if INFO_CPU_OLD.total_sec == 0 {
            INFO_CPU_OLD.child_cpu_sys = c.child_cpu_sys;
            INFO_CPU_OLD.total_sec = c.total_sec;
            INFO_CPU_OLD.child_cpu_user = c.child_cpu_user;
            INFO_CPU_OLD.cpu_sys = c.cpu_sys;
            INFO_CPU_OLD.cpu_user = c.cpu_user;

            debug!("redis info cpu is first running");
            return Ok(());
        }
        else {
            input.child_cpu_sys = c.child_cpu_sys - INFO_CPU_OLD.child_cpu_sys;
            input.child_cpu_user = c.child_cpu_user - INFO_CPU_OLD.child_cpu_user;
            input.cpu_sys = c.cpu_sys - INFO_CPU_OLD.cpu_sys;
            input.cpu_user = c.cpu_user - INFO_CPU_OLD.cpu_user;
            input.total_sec = c.total_sec - INFO_CPU_OLD.total_sec;

            INFO_CPU_OLD.child_cpu_sys = c.child_cpu_sys;
            INFO_CPU_OLD.total_sec = c.total_sec;
            INFO_CPU_OLD.child_cpu_user = c.child_cpu_user;
            INFO_CPU_OLD.cpu_sys = c.cpu_sys;
            INFO_CPU_OLD.cpu_user = c.cpu_user;
        }
    }

    let pg_query = COLLECT_COMMANDLINE_MAP.get(&CollectCommand::CommonCpu).unwrap();

    let mut collect_conn_item = {
        get_redis_global().pools.collect_pool.get_owned(())?
    };

    let collect_conn = collect_conn_item.get_value();

    collect_conn.execute(&pg_query,
         &[&server_id, 
                &make_pg_numeric(input.cpu_sys / input.total_sec as f64), 
                &make_pg_numeric(input.cpu_user / input.total_sec as f64), 
                &make_pg_numeric(input.child_cpu_sys / input.total_sec as f64), 
                &make_pg_numeric(input.child_cpu_user / input.total_sec as f64)])?;
    Ok(())
}

pub fn config_get_all_handle(server_id : i32, val : String) -> Result<(),Box<dyn Error>> {
    let parsing_data = parsing_config_get_all(val)?;
    let mut config_iter = parsing_data.iter();

    let mut collect_conn_item = {
        get_redis_global().pools.collect_pool.get_owned(())?
    };

    let collect_conn = collect_conn_item.get_value();

    let mut t = collect_conn.trans()?;
    let pg_query = COLLECT_COMMANDLINE_MAP.get(&CollectCommand::CommonConfigAll).unwrap();
    
    loop {
        let seq = config_iter.next();
        if seq.is_none() {
            let _ = t.commit();
            break;
        }

        let conf = seq.unwrap();
        let exec_ret = t.execute(&pg_query, &[&server_id, &conf.name, &conf.value]);

        if exec_ret.is_err() {
            let _ = t.rollback();
            return Err(exec_ret.unwrap_err());
        }
    }

    Ok(())
}

pub fn db_size_handle(server_id : i32, val : String) -> Result<(),Box<dyn Error>> {
    let pg_query = COLLECT_COMMANDLINE_MAP.get(&CollectCommand::RedisDbSize).unwrap();
    let dbsize = parsing_dbsize(val)?;

    let mut collect_conn_item = {
        get_redis_global().pools.collect_pool.get_owned(())?
    };

    let collect_conn = collect_conn_item.get_value();

    collect_conn.execute(&pg_query, &[&server_id, &dbsize])?;
    
    Ok(())
}

pub fn info_stats_handle(server_id : i32, val : String) -> Result<(),Box<dyn Error>> {
    let pg_query = COLLECT_COMMANDLINE_MAP.get(&CollectCommand::RedisInfoStat).unwrap();
    let c = parsing_info_stat(val)?;

    let mut collect_conn_item = {
        get_redis_global().pools.collect_pool.get_owned(())?
    };

    let collect_conn = collect_conn_item.get_value();

    collect_conn.execute(&pg_query, &[&server_id,
        &c.total_connections_received,
        &c.total_commands_processed,
        &c.instantaneous_ops_per_sec,
        &c.total_net_input_bytes,
        &c.total_net_output_bytes,
        &make_pg_numeric(c.instantaneous_input_kbps),
        &make_pg_numeric(c.instantaneous_output_kbps),
        &c.rejected_connections,
        &c.sync_full,
        &c.sync_partial_ok,
        &c.sync_partial_err,
        &c.expired_keys,
        &c.evicted_keys,
        &c.keyspace_hits,
        &c.keyspace_misses,
        &c.pubsub_channels,
        &c.pubsub_patterns,
        &c.latest_fork_usec,
        &c.migrate_cached_sockets,
        &c.slave_expires_tracked_keys,
        &c.active_defrag_hits,
        &c.active_defrag_misses,
        &c.active_defrag_key_hits,
        &c.active_defrag_key_misses])?;
    Ok(())
}