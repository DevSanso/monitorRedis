use std::collections::HashMap;
use std::env;

macro_rules! add_arg {
    ($map:expr, $key:expr, $desc:expr, $required:expr) => {
        $map.insert($key, ($desc, $required));
    };
}

macro_rules! parse_arg {
    ($args:expr, $key:expr, $map:expr) => {
        if let Some(pos) = $args.iter().position(|arg| arg == $key) {
            if pos + 1 < $args.len() {
                Some($args[pos + 1].clone())
            } else {
                eprintln!("not setting value {}", $map.get($key).unwrap().0);
                None
            }
        } else {
            if $map.get($key).unwrap().1 {
                eprintln!("need config key{}", $map.get($key).unwrap().0);
                None 
            } else {
                Some(String::new()) 
            }
        }
    };
}

fn check_required_args_count(args: &[String], arg_descriptions: &HashMap<&str, (&str, bool)>) -> Option<String> {
    let required_args_count = arg_descriptions.values().filter(|&&(_, required)| required).count();
    if args.len() < required_args_count * 2 + 1 {
        return Some(format!("There are not enough arguments. At least {} are required.", required_args_count * 2 + 1));
    }
    None
}

fn print_help(arg_descriptions: &HashMap<&str, (&str, bool)>) {
    println!("Usage: Program name [option]");
    println!("Option:");
    for (key, (desc, _)) in arg_descriptions {
        println!("  {}  : {}", key, desc);
    }
}

#[derive(Default, Clone)]
pub struct Args {
    pub server_type : String,
    pub server_id : i32,
    pub log_level : String,
    pub log_file_path : String,
    pub conf_path : String
}

impl Args {
    pub fn new() -> Option<Self> {
        let args: Vec<String> = env::args().collect();

        let mut arg_descriptions = HashMap::new();

        add_arg!(arg_descriptions, "--serverType", "server type", true);
        add_arg!(arg_descriptions, "--server_id", "server id", true);
        add_arg!(arg_descriptions, "--log_level", "log level", true);
        add_arg!(arg_descriptions, "--log_file_path", "log file path", true);
        add_arg!(arg_descriptions, "--conf_path", "connection config path", false);
        add_arg!(arg_descriptions, "--help", "print help", false);

        if args.contains(&"--help".to_string()) {
            print_help(&arg_descriptions);
            return None;
        }

        let mut this = Args::default();

        check_required_args_count(&args, &arg_descriptions);

        this.server_type = parse_arg!(args, "--serverType", &arg_descriptions)?;
        this.log_level = parse_arg!(args, "--log_level", &arg_descriptions)?;
        this.log_file_path = parse_arg!(args, "--log_file_path", &arg_descriptions)?;
        this.conf_path = parse_arg!(args, "--conn_path", &arg_descriptions)?;
        let server_id_str = parse_arg!(args, "--server_id", &arg_descriptions)?;

        this.server_id = server_id_str.parse().unwrap_or_else(|_| {
            eprintln!("--server_id only input number.");
            -1
        });

        if this.server_id == -1 {
            return None;
        }

        Some(this)
    }

}