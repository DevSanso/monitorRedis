use simplelog::*;
use log::*;
pub struct LoggerConfig {
    level : String,
    path : Option<String>
}

impl LoggerConfig {
    pub fn new(level : String, path : Option<String>) -> Self {
        LoggerConfig {
            level,
            path
        }
    }
}

fn convert_string_to_filter(level: &String) -> LevelFilter {
    match level.as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Error
    }
}

pub fn init_logger(cfgs : Vec<LoggerConfig>) -> Result<(), log::SetLoggerError> {
    let mut loggers = cfgs.iter().fold( Vec::<Box<dyn SharedLogger + 'static>>::new(), |mut acc, cfg| {
        
        if cfg.path.is_some() {
            acc.push(WriteLogger::new(
                convert_string_to_filter(&cfg.level),
                Config::default(),
                std::fs::File::create(cfg.path.as_ref().unwrap()).unwrap(),
            ));
        }
        acc
    });

    loggers.push(TermLogger::new(LevelFilter::Info, Config::default(), TerminalMode::Mixed, ColorChoice::Never));

    let ret = CombinedLogger::init(loggers);
    ret
}