use std::{io::{self, BufReader, BufRead}, path::PathBuf};
use csv;
use clap::{AppSettings, Clap};
use log::{LevelFilter};
use log4rs::{
    Config,
    Handle,
    append::console::{ConsoleAppender, Target},
    config::{Appender, Root},
    encode::{Encode, pattern::PatternEncoder}
};

mod person;
use person::Person;


#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(short, long, default_value = ",")]
    input_delimiter: char,

    #[clap(short, long, default_value = ",")]
    output_delimiter: char,

    #[clap(long, about = "Input contains header row")]
    has_headers: bool,

    #[clap(short, long)]
    attrs: Vec<String>,

    #[clap(name = "FILE", parse(from_os_str), about = "CSV input files...")]
    files: Vec<PathBuf>,
}


fn read_input_files(opts: &Opts, people: &mut Vec<Person>) -> io::Result<()> {

    for path in opts.files.iter() {

        let input: Box<dyn BufRead> = match path.to_str().unwrap() {
            "-" => Box::new(BufReader::new(io::stdin())),
            x => Box::new(BufReader::new(std::fs::File::open(x).unwrap()))
        };

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(opts.input_delimiter as u8)
            .has_headers(opts.has_headers)
            .trim(csv::Trim::All)
            .from_reader(input);

        for result in reader.deserialize::<Person>() {
            match result {
                Err(e) => log::warn!("Problem deserializing person: {}", e),
                Ok(p) => people.push(p)
            }
        }
    }

    Ok(())
}

fn set_console_logger() -> Result<Handle, log::SetLoggerError> {

    let encoder: Box<dyn Encode> = Box::new(PatternEncoder::new("{d} {h({l})} {t} - {m}{n}"));
    let stderr = ConsoleAppender::builder().encoder(encoder).target(Target::Stderr).build();
    let config = Config::builder()
        .appender(
            Appender::builder().build("stderr", Box::new(stderr))
        )
        .build(Root::builder().appender("stderr").build(LevelFilter::Warn))
        .unwrap();

    log4rs::init_config(config)
}


fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();
    let mut people: Vec<Person> = vec![];
    let attrs: Vec<&str> = opts.attrs.iter().map(String::as_str).collect();

    set_console_logger().unwrap();

    read_input_files(&opts, &mut people)?;

    people.sort_by(|a, b| Person::cmp_order_by_attrs(a, b, &attrs));

    let mut writer = csv::WriterBuilder::new()
        .delimiter(opts.output_delimiter as u8)
        .has_headers(opts.has_headers)
        .terminator(csv::Terminator::CRLF)
        .from_writer(io::stdout());

    for result in people.iter().map(|p| writer.serialize(p)) {
        match result {
            Err(e) => log::warn!("Problem serializing person: {}", e),
            _ => ()
        }
    }

    writer.flush()
}
