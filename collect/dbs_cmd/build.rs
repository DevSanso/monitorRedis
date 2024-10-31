use serde_json;
use std::collections::HashMap;
use std::fs;
use std::io::Write;
use std::path::Path;
use std::process::Command;

static CMD_OUTPUT_FILE: &'static str = "src/lib.rs";

fn get_redis_command_json_file() -> Vec<u8> {
    include_bytes!("../assets/command/redis.json").to_vec()
}

fn get_pg_command_json_file() -> Vec<u8> {
    include_bytes!("../assets/command/collect.json").to_vec()
}

fn get_sqlite_command_json_file() -> Vec<u8> {
    include_bytes!("../assets/command/manage.json").to_vec()
}

fn write_link_submod(root_mod: &mut Vec<u8>, sub_mod: &'_ str) {
    write!(root_mod, "pub mod {};\n", sub_mod);
}

fn write_dep_module(root_mod: &mut Vec<u8>, third_mod_name: &'_ str) {
    write!(root_mod, "use {};\n", third_mod_name);
}

fn create_or_clean_mod_code(path: String) -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new(&path).exists() {
        let f = fs::File::create(&path)?;
        drop(f)
    }

    let mut f = fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(path.as_str())?;
    Ok(())
}

fn rewrite_mod_code(path: String, data: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut f = fs::OpenOptions::new()
        .write(true)
        .append(true)
        .open(path.as_str())?;
    f.write(data.as_bytes())?;
    Ok(())
}

fn create_enum_code(enum_name: &str, list: &Vec<(String, String)>) -> String {
    let mut ret = String::new();
    let mut count = 1;

    ret.push_str(
        format!(
            "#[derive(Eq, PartialEq, Hash)]\npub enum {} {{\n",
            enum_name
        )
        .as_str(),
    );
    for ele in list {
        match count < list.len() {
            true => ret.push_str(format!("{},\n", ele.0).as_str()),
            false => ret.push_str(format!("{}", ele.0).as_str()),
        };

        count += 1;
    }

    ret.push_str("\n}");
    ret
}

fn create_static_hashmap(
    hashmap_name: &str,
    enum_name: &str,
    list: &Vec<(String, String)>,
) -> String {
    let mut ret = String::new();
    let define = format!(
        "pub static {} : once_cell::sync::Lazy<HashMap<{}, &'_ str>>",
        str::to_uppercase(hashmap_name),
        enum_name
    );

    let mut init_func = String::from("|| {\n");
    init_func.push_str(
        format!(
            "let mut {}_internal = HashMap::new();\n",
            hashmap_name.to_lowercase()
        )
        .as_str(),
    );

    for ele in list {
        init_func.push_str(
            format!(
                "{}_internal.insert({}::{},\"{}\");\n",
                hashmap_name.to_lowercase(),
                enum_name,
                ele.0,
                ele.1
            )
            .as_str(),
        );
    }
    init_func.push_str(format!("{}_internal \n}}", hashmap_name.to_lowercase()).as_str());

    let mut lazy_new = format!("once_cell::sync::Lazy::new({})", init_func);

    ret.push_str(format!("{} = {};", define, lazy_new).as_str());
    ret
}

fn get_redis_command_code() -> Result<(String, String), Box<dyn std::error::Error>> {
    const ENUM_NAME: &'static str = "RedisCommand";
    const MAP_NAME: &'static str = "REIDS_COMMANDLINE_MAP";

    let redis_data = get_redis_command_json_file();
    let cmds: Vec<HashMap<String, String>> = serde_json::from_slice(&redis_data)?;

    let cmd_tups = cmds
        .iter()
        .fold(Vec::<(String, String)>::new(), |mut acc, ele| {
            ele.keys().for_each(|k| {
                acc.push((k.clone(), ele.get(k).unwrap().clone()));
            });
            acc
        });

    let enum_code = create_enum_code(ENUM_NAME, &cmd_tups);
    let map_code = create_static_hashmap(MAP_NAME, ENUM_NAME, &cmd_tups);

    Ok((enum_code, map_code))
}

fn get_rdb_command_code(
    enum_name: &'static str,
    map_name: &'static str,
    rdb_sql_file: Vec<u8>,
) -> Result<(String, String), Box<dyn std::error::Error>> {
    let cmds: Vec<HashMap<String, Vec<String>>> = serde_json::from_slice(&rdb_sql_file)?;

    let cmd_tups = cmds
        .iter()
        .fold(Vec::<(String, String)>::new(), |mut acc, ele| {
            ele.keys().for_each(|k| {
                acc.push((k.clone(), ele.get(k).unwrap().join(" ")));
            });
            acc
        });

    let enum_code = create_enum_code(enum_name, &cmd_tups);
    let map_code = create_static_hashmap(map_name, enum_name, &cmd_tups);

    Ok((enum_code, map_code))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut output_data: Vec<u8> = vec![];
    create_or_clean_mod_code(format!(
        "{}/{}",
        env!("CARGO_MANIFEST_DIR"),
        CMD_OUTPUT_FILE
    ))?;

    write_dep_module(&mut output_data, "once_cell");
    write_dep_module(&mut output_data, "std::collections::HashMap");

    let redis_codes = get_redis_command_code()?;
    let pg_codes = get_rdb_command_code(
        "CollectCommand",
        "COllECT_COMMANDLINE_MAP",
        get_pg_command_json_file(),
    )?;
    let sqlite_codes = get_rdb_command_code(
        "ManageCommand",
        "MANAGE_COMMANDLINE_MAP",
        get_sqlite_command_json_file(),
    )?;

    let all_code = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}",
        std::str::from_utf8(&output_data.as_slice()).unwrap(),
        redis_codes.0,
        redis_codes.1,
        pg_codes.0,
        pg_codes.1,
        sqlite_codes.0,
        sqlite_codes.1
    );
    let output_file = format!("{}/{}", env!("CARGO_MANIFEST_DIR"), CMD_OUTPUT_FILE);
    rewrite_mod_code(output_file.clone(), format!("{}", all_code))?;

    let _output = Command::new("rustfmt")
        .arg(output_file.as_str())
        .output()?;
    Ok(())
}
