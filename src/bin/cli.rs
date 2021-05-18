use std::{collections::VecDeque, path::PathBuf};
use clap::{AppSettings, Clap};
use log::LevelFilter;

use tokio::io;

use homework::person::Person;
use homework::serialization::StructFieldDeserialize;
use homework::sort_direction::SortDirection;
use homework::io::*;
use homework::log::*;


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
    set_console_logger(LevelFilter::Warn).unwrap();

    let opts: Opts = Opts::parse();
    let fields = sorting_fields(&opts);
    let mut people: Vec<Person> = vec![];
    let output_field_separator = match opts.output_field_separator {
        Some(o) => o,
        None => opts.input_field_separator
    };

    if opts.available_fields {
        println!("{}", Person::struct_fields().join(", "));
        return Ok(());
    }

    read_input_files(
        &opts.files,
        opts.input_field_separator,
        &opts.input_field_separator_mappings,
        opts.input_has_header,
        &opts.input_has_header_mappings,
        &mut people
    ).await?;

    people.sort_by(|a, b| Person::cmp_order_by_fields(a, b, &fields));

    write_output(output_field_separator, opts.output_has_header, &people)
}
