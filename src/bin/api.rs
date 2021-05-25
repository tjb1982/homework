use std::{net::{SocketAddr, ToSocketAddrs}, path::PathBuf};
use std::env;
use log::LevelFilter;
use warp::Filter;
use clap::{AppSettings, Clap};

use homework::api::filters;
use homework::api::models;


#[derive(Clap, Clone)]
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


impl From<Opts> for models::DbOpts {
    fn from(opts: Opts) -> Self {
        Self::new(
            opts.files,
            opts.input_field_separator,
            opts.input_field_separator_mappings.clone(),
            opts.input_has_header,
            opts.input_has_header_mappings.clone(),
        )
    }
}


fn cors() -> warp::cors::Builder {
    warp::cors()
        .allow_any_origin()
        .allow_headers(vec!["content-type"])
        .allow_methods(vec!["POST"])
}


#[tokio::main]
async fn main() {
    homework::log::set_console_logger(LevelFilter::Info).unwrap();

    let opts: Opts = Opts::parse();
    let db = models::init_db(opts.clone().into()).await;

    let addr: Vec<SocketAddr> = opts.hostname
        .to_socket_addrs()
        .expect(format!("Bad hostname: {}", opts.hostname).as_str())
        .collect();

    let api = warp::options().map(warp::reply).or(filters::records(db))
        .with(cors())
        .with(warp::log("homework"));

    warp::serve(api).run(*addr.first().expect("Address not found.")).await;
}
