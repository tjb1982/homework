use std::{io::{self, BufReader, BufRead}, path::PathBuf};
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

mod person;
use person::Person;

mod struct_fields;
use struct_fields::StructFieldDeserialize;

mod sort_direction;
use crate::sort_direction::SortDirection;


#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {

    #[clap(short, long, about = "Display all available sortable fields and exit.")]
    available_fields: bool,

    #[clap(short = 'S', long = "default-field-separator", default_value = ",")]
    input_separator: char,

    #[clap(short = 's', about = "Map separators to each respective input file (falling back to the default).")]
    input_separator_mappings: Vec<char>,

    #[clap(short, long, default_value = ",")]
    output_separator: char,

    #[clap(short = 'E', long, about = "All inputs contain a header row, unless otherwise indicated.")]
    input_has_header: bool,

    #[clap(short = 'e', about = "Map `has_header` to each respective input file (falling back to the default: false).")]
    input_has_header_mappings: Vec<bool>,

    #[clap(short = 't', long, about = "Output will contain a header row.")]
    output_has_header: bool,

    #[clap(short = 'f', long, about = "Sequential list of fields to sort the output.")]
    fields: Vec<String>,

    #[clap(short = 'D', long, default_value = "asc")]
    sort_direction: &'static SortDirection,

    #[clap(short = 'd', long)]
    sort_direction_mappings: Vec<&'static SortDirection>,

    #[clap(name = "FILE", parse(from_os_str), about = "CSV input files...")]
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


fn read_input_files(opts: &Opts, people: &mut Vec<Person>) -> io::Result<()> {

    let mut input_separator_mappings = opts.input_separator_mappings.clone();
    let mut input_has_header_mappings = opts.input_has_header_mappings.clone();

    input_separator_mappings.reverse();
    input_has_header_mappings.reverse();

    for path in opts.files.iter() {

        let input: Box<dyn BufRead> = match path.to_str().unwrap() {
            "-" => Box::new(BufReader::new(io::stdin())),
            x => Box::new(BufReader::new(std::fs::File::open(x).unwrap()))
        };

        let input_separator = match input_separator_mappings.pop() {
            None => opts.input_separator,
            Some(c) => c
        } as u8;

        let input_has_header = match input_has_header_mappings.pop() {
            None => opts.input_has_header,
            Some(b) => b
        };

        let mut reader = csv::ReaderBuilder::new()
            .delimiter(input_separator)
            .has_headers(input_has_header)
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


fn write_output(opts: &Opts, people: &Vec<Person>) -> Result<(), io::Error> {
    let mut writer = csv::WriterBuilder::new()
        .delimiter(opts.output_separator as u8)
        .has_headers(opts.output_has_header)
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

fn sorting_fields(opts: &Opts) -> Vec<(&str, &SortDirection)> {
    let mut sort_direction_mappings = opts.sort_direction_mappings.clone();
    let field_names: Vec<&str> = opts.fields.iter()
        .map(String::as_str)
        .collect();

    sort_direction_mappings.reverse();

    field_names.iter().map(|&name| match sort_direction_mappings.pop() {
        Some(sd) => (name, sd),
        None => (name, opts.sort_direction)
    }).collect()
}


fn main() -> io::Result<()> {
    set_console_logger().unwrap();

    let opts: Opts = Opts::parse();
    let mut people: Vec<Person> = vec![];
    let fields = sorting_fields(&opts);

    if opts.available_fields {
        println!("{:?}", Person::struct_fields());
        return Ok(());
    }

    read_input_files(&opts, &mut people)?;

    people.sort_by(|a, b| Person::cmp_order_by_fields(a, b, &fields));

    write_output(&opts, &people)
}
