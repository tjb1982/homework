use log4rs::{
    Config,
    Handle,
    append::console::{ConsoleAppender, Target},
    config::{Appender, Root},
    encode::{Encode, pattern::PatternEncoder}
};

pub fn set_console_logger(level_filter: log::LevelFilter) -> Result<Handle, log::SetLoggerError> {

    let encoder: Box<dyn Encode> = Box::new(PatternEncoder::new("{d} {h({l})} {t} - {m}{n}"));
    let stderr = ConsoleAppender::builder().encoder(encoder).target(Target::Stderr).build();
    let config = Config::builder()
        .appender(
            Appender::builder().build("stderr", Box::new(stderr))
        )
        .build(Root::builder().appender("stderr").build(level_filter))
        .unwrap();

    log4rs::init_config(config)
}
