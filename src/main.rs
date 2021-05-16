use std::{io::{self, BufReader, BufRead}, path::PathBuf};
use csv;

mod person;
use person::Person;
use clap::{AppSettings, Clap};


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

        people.extend(
            reader.deserialize::<Person>()
                .map(Result::unwrap).collect::<Vec<Person>>()
        );

    }

    Ok(())
}


fn main() -> io::Result<()> {
    let opts: Opts = Opts::parse();
    let mut people: Vec<Person> = vec![];
    let attrs: Vec<&str> = opts.attrs.iter().map(String::as_str).collect();

    read_input_files(&opts, &mut people)?;

    people.sort_by(|a, b| Person::cmp_order_by_attrs(a, b, &attrs));

    let mut writer = csv::WriterBuilder::new()
        .delimiter(opts.output_delimiter as u8)
        .has_headers(opts.has_headers)
        .terminator(csv::Terminator::CRLF)
        .from_writer(io::stdout());

    for person in people {
        writer.serialize(person)?;
    }

    writer.flush()
}
