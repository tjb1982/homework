mod person;
mod serialization;
mod sort_direction;

use std::{collections::VecDeque, error::Error, io as stdio, path::PathBuf};
use csv;
use clap::{AppSettings, Clap};
use log::LevelFilter;
use log4rs::{
    Config,
    Handle,
    append::console::{ConsoleAppender, Target},
    config::{Appender, Root},
    encode::{Encode, pattern::PatternEncoder}
};

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

use person::Person;
use serialization::StructFieldDeserialize;
use crate::sort_direction::SortDirection;


#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {

    #[clap(short, long, about = "Display all available sorting fields and exit")]
    available_fields: bool,

    #[clap(short = 'S', long, default_value = ",")]
    input_field_separator: char,

    #[clap(short = 's', long = "input-field-separator-mapping", about = "Map `--field-separator` to each respective input file (any remaining unmapped files fall back to `--field-separator`)")]
    input_field_separator_mappings: Vec<char>,

    #[clap(short, long, about = "Separator to use for the output")]
    output_field_separator: Option<char>,

    #[clap(short = 'E', long, about = "Inputs contain header row")]
    input_has_header: bool,

    #[clap(short = 'e', long = "input-has-header-mapping", about = "Map `--input-has-header` to each respective input file (any remaining unmapped files fall back to `--input-has-header`)")]
    input_has_header_mappings: Vec<bool>,

    #[clap(short = 't', long, about = "Output will contain a header row")]
    output_has_header: bool,

    #[clap(short = 'f', long = "field", about = "Sequential list of fields to sort the output")]
    fields: Vec<String>,

    #[clap(short = 'D', long, default_value = "asc")]
    sort_direction: SortDirection,

    #[clap(short = 'd', long = "sort-direction-mapping", about = "Sequential list of sort directions, mapped to each provided `--field` (any remaining unmapped `--fields` fall back to `--sort-direction`)")]
    sort_direction_mappings: Vec<SortDirection>,

    #[clap(name = "FILE", parse(from_os_str), about = "CSV input files...", required = true)]
    files: Vec<PathBuf>,
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


async fn read_input_file(input_field_separator: char, input_has_header: bool, path: &PathBuf) -> io::Result<Vec<Person>>
{
    let mut input = String::new();
    let mut people: Vec<Person> = vec![];
        
    let _ = match path.to_str().unwrap() {
        "-" => io::stdin().read_to_string(&mut input).await,
        x => File::open(x).await?.read_to_string(&mut input).await
    };

    let mut reader = csv::ReaderBuilder::new()
        .delimiter(input_field_separator as u8)
        .has_headers(input_has_header)
        .trim(csv::Trim::All)
        .from_reader(input.as_str().as_bytes());

    for result in reader.deserialize::<Person>() {
        match result {
            Err(e) => log::warn!("Problem deserializing person: {}", e),
            Ok(p) => people.push(p)
        }
    }
    Ok(people)
}


async fn read_input_files(opts: &Opts, people: &mut Vec<Person>) -> io::Result<()>
{

    let mut input_field_separator_mappings = VecDeque::from(opts.input_field_separator_mappings.clone());
    let mut input_has_header_mappings = opts.input_has_header_mappings.clone();
    let mut futures: Vec<_> = vec![];

    for path in opts.files.iter() {

        let input_field_separator = match input_field_separator_mappings.pop_front() {
            None => opts.input_field_separator,
            Some(c) => c
        };

        let input_has_header = match input_has_header_mappings.pop() {
            None => opts.input_has_header,
            Some(b) => b
        };

        futures.push(read_input_file(
            input_field_separator,
            input_has_header,
            path,
        ));
    }

    for f in futures {
        people.extend(f.await?);
    }

    Ok(())
}


fn csv_err_is_broken_pipe(e: &csv::Error) -> bool
{
    e.source().unwrap().downcast_ref::<stdio::Error>().unwrap().kind() == stdio::ErrorKind::BrokenPipe
}


fn write_output(opts: &Opts, people: &Vec<Person>) -> Result<(), stdio::Error> {

    let output_field_separator = match opts.output_field_separator {
        Some(o) => o,
        None => opts.input_field_separator
    };

    let mut writer = csv::WriterBuilder::new()
        .delimiter(output_field_separator as u8)
        .has_headers(opts.output_has_header)
        .terminator(csv::Terminator::CRLF)
        .from_writer(stdio::stdout());

    for result in people.iter().map(|p| writer.serialize(p)) {
        match result {
            Err(e) if csv_err_is_broken_pipe(&e) => {
                log::warn!("{}", e);
                return Ok(())
            },
            Err(e) => log::warn!("Problem serializing person: {}", e),
            _ => ()
        }
    }

    writer.flush()

}


fn sorting_fields(opts: &Opts) -> Vec<(&str, SortDirection)> {
    let mut sort_direction_mappings = VecDeque::from(opts.sort_direction_mappings.clone());
    let field_names: Vec<&str> = opts.fields.iter()
        .map(String::as_str)
        .collect();

    field_names.iter().map(|&name| match sort_direction_mappings.pop_front() {
        Some(sd) => (name, sd),
        None => (name, opts.sort_direction)
    }).collect()
}


#[tokio::main]
async fn main() -> io::Result<()> {
    set_console_logger().unwrap();

    let opts: Opts = Opts::parse();
    let fields = sorting_fields(&opts);
    let mut people: Vec<Person> = vec![];

    if opts.available_fields {
        println!("{}", Person::struct_fields().join(", "));
        return Ok(());
    }

    read_input_files(&opts, &mut people).await?;

    people.sort_by(|a, b| Person::cmp_order_by_fields(a, b, &fields));

    write_output(&opts, &people)
}
