use std::{collections::VecDeque, error::Error, io as stdio, path::PathBuf};
use csv;

use tokio::fs::File;
use tokio::io::{self, AsyncReadExt};

use crate::person::Person;


fn csv_err_is_broken_pipe(e: &csv::Error) -> bool
{
    e.source().unwrap().downcast_ref::<stdio::Error>().unwrap().kind() == stdio::ErrorKind::BrokenPipe
}


pub fn write_output(
    output_field_separator: char,
    output_has_header: bool,
    people: &Vec<Person>
) -> Result<(), stdio::Error>
{

    let mut writer = csv::WriterBuilder::new()
        .delimiter(output_field_separator as u8)
        .has_headers(output_has_header)
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


async fn read_input_file(
    input_field_separator: char,
    input_has_header: bool,
    path: &PathBuf,
) -> io::Result<Vec<Person>>
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


pub async fn read_input_files(
    files: &Vec<PathBuf>,
    input_field_separator: char,
    input_field_separator_mappings: &Vec<char>,
    input_has_header: bool,
    input_has_header_mappings: &Vec<bool>,
    people: &mut Vec<Person>
) -> io::Result<()>
{

    let mut input_field_separator_mappings = VecDeque::from(input_field_separator_mappings.clone());
    let mut input_has_header_mappings = input_has_header_mappings.clone();
    let mut futures: Vec<_> = vec![];

    for path in files.iter() {

        let input_field_separator = match input_field_separator_mappings.pop_front() {
            None => input_field_separator,
            Some(c) => c
        };

        let input_has_header = match input_has_header_mappings.pop() {
            None => input_has_header,
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
