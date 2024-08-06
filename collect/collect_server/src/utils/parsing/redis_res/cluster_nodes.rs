use std::error::Error;
use std::num::ParseIntError;

use core::utils::parsing::common::*;
use core::utils_new_error;
use core::utils_inherit_error;

#[derive(Default)]
pub struct ClusterNode {
    pub node_id : String,
    pub ip : String,
    pub port : i64,
    pub cport : i64,
    pub node_type : String,
    pub master_node : Option<String>,
    pub ping_send : i64,
    pub ping_recv : i64,
    pub ping_epoch : i64,
    pub connect_type : String,
    pub slots : Vec<i32>
}

pub type ClusterNodes = Vec<ClusterNode>;

fn parsing_cluster_node(line : &'_ str) -> Result<ClusterNode, Box<dyn Error>> {
    let mut ret = ClusterNode::default();
    let mut index = 0;
    
    for data in line.split_whitespace() {
        match index {
            0 => ret.node_id = String::from(data),
            1 => {
                let s = split_colon_tuple(data);
                if s.is_err() {
                    return utils_inherit_error!(data, GetDataCastError , "", s.err().unwrap());
                }
                let (ip, ports) = s.unwrap();

                ret.ip = ip.to_string();
                let mut port = None;
                let mut cport = None;
                {
                    let mut port_split = ports.split("@");
                    port = port_split.next();
                    cport = port_split.next();
                }

                if port.is_none() || cport.is_none() {
                    return utils_new_error!(data, GetDataCastError, String::from("cluster nodes port or cport is null"));
                }
                
                ret.port = match port.unwrap().parse::<i64>() {
                    Ok(ok) => ok,
                    Err(_) => return utils_new_error!(data, GetDataCastError, format!("port convert failed {}", port.unwrap()))
                };

                ret.cport = match cport.unwrap().parse::<i64>() {
                    Ok(ok) => ok,
                    Err(_) => return utils_new_error!(data, GetDataCastError, format!("port convert failed {}", cport.unwrap()))
                };
            },
            2 => ret.node_type = String::from(data),
            3 => {
                ret.master_node = if data == "-" {
                    None
                }else{
                    Some(String::from(data))
                };
            },
            4 => {
                ret.ping_send = match data.parse::<i64>() {
                    Ok(ok) => ok,
                    Err(_) => return utils_new_error!(data, GetDataCastError, format!("ping_send convert failed {}", data))
                };
            },
            5 => {
                ret.ping_recv = match data.parse::<i64>() {
                    Ok(ok) => ok,
                    Err(_) => return utils_new_error!(data, GetDataCastError, format!("ping_recv convert failed {}", data))
                };
            },
            6 => {
                ret.ping_epoch = match data.parse::<i64>() {
                    Ok(ok) => ok,
                    Err(_) => return utils_new_error!(data, GetDataCastError, format!("ping_epoch convert failed {}", data))
                };
            },
            7 => ret.connect_type = String::from(data),
            _ => {
                let slot = match data.parse::<i32>() {
                    Ok(ok) => ok,
                    Err(_) => return utils_new_error!(data, GetDataCastError, format!("ping_epoch convert failed {}", data))
                };
                ret.slots.push(slot);
            }
        }

        index += 1;
    }

    Ok(ret)
}

pub fn parsing_cluster_nodes(res : String) -> Result<ClusterNodes, Box<dyn Error>> {
    let mut ret = Vec::new();

    for line in res.trim().split("\n") {
        if line.len() == 0 {continue;}
        ret.push(parsing_cluster_node(line)?);
    }

    Ok(ret)
}
