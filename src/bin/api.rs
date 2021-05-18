use std::{net::{SocketAddr, ToSocketAddrs}, path::PathBuf};
use std::env;
use log::LevelFilter;
use warp::Filter;
use clap::{AppSettings, Clap};
use tokio::io;


#[derive(Clap)]
#[clap(version = env!("CARGO_PKG_VERSION"), author = env!("CARGO_PKG_AUTHORS"))]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct Opts {

    #[clap(short = 'S', long, default_value = ",")]
    input_field_separator: char,

    #[clap(short = 's', long = "input-field-separator-mapping", about = "Map `--field-separator` to each respective input file (any remaining unmapped files fall back to `--field-separator`)")]
    input_field_separator_mappings: Vec<char>,

    #[clap(short = 'E', long, about = "Inputs contain header row")]
    input_has_header: bool,

    #[clap(short = 'e', long = "input-has-header-mapping", about = "Map `--input-has-header` to each respective input file (any remaining unmapped files fall back to `--input-has-header`)")]
    input_has_header_mappings: Vec<bool>,

    #[clap(name = "FILE", parse(from_os_str), about = "CSV input files...", required = true)]
    files: Vec<PathBuf>,

    #[clap(short = 'H', long = "hostname", about = "Hostname to serve this API on")]
    hostname: String,
}


#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();
    let db = models::init_db(&opts).await;

    homework::log::set_console_logger(LevelFilter::Info).unwrap();

    let addr: Vec<SocketAddr> = opts.hostname
        .to_socket_addrs()
        .expect(format!("Bad hostname: {}", opts.hostname).as_str())
        .collect();

    let hi = warp::path("records")
        .and(warp::path::param())
        .and(warp::header("user-agent"))
        .map(|param: String, agent: String| {
            format!("Hello {}, whose agent is {}", param, agent)
        });
    let api = hi.with(warp::log("homework"));
    warp::serve(api).run(addr[0]).await;
}


mod models {
    use super::*;
    use serde::{Deserialize, Serialize};
    use std::sync::Arc;
    use tokio::sync::Mutex;
    use homework::person::Person;
    use homework::io::read_input_files;

    /// In-memory "database"
    pub type Db = Arc<Mutex<Vec<Person>>>;

    pub async fn init_db(opts: &Opts) -> Db {
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


    // The query parameters for list_todos.
    #[derive(Debug, Deserialize)]
    pub struct ListOptions {
        pub offset: Option<usize>,
        pub limit: Option<usize>,
    }
}