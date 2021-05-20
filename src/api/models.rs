use serde::{Deserialize};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::Mutex;
use crate::{person::Person, sorting::SortDirection};
use crate::io::read_input_files;


/// In-memory "database"
pub type Db = Arc<Mutex<Vec<Person>>>;


pub struct DbOpts {
    files: Vec<PathBuf>,
    input_field_separator: char,
    input_field_separator_mappings: Vec<char>,
    input_has_header: bool,
    input_has_header_mappings: Vec<bool>,
}


impl DbOpts {
    pub fn new(
        files: Vec<PathBuf>,
        input_field_separator: char,
        input_field_separator_mappings: Vec<char>,
        input_has_header: bool,
        input_has_header_mappings: Vec<bool>,
    ) -> Self
    {
        Self {
            files,
            input_field_separator,
            input_field_separator_mappings,
            input_has_header,
            input_has_header_mappings,
        }
    }
}


#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct ListOptions {
    pub direction: Option<SortDirection>,
    pub page: Option<usize>,
    pub per_page: Option<usize>,
}


pub async fn init_db (
    opts: DbOpts
) -> Db {
    let mut people: Vec<Person> = vec![];

    let _ = read_input_files(
        &opts.files,
        opts.input_field_separator,
        &opts.input_field_separator_mappings,
        opts.input_has_header,
        &opts.input_has_header_mappings,
        &mut people
    ).await;

    Arc::new(Mutex::new(people))
}
