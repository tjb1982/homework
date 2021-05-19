use warp::{Buf, Filter, Rejection};

use crate::api::models::{self, Db};
use crate::api::handlers;
use crate::person::Person;


/// I.e., 2 MiB
const MAX_BYTES: u64 = (1 << 20) * 2;

/// A filter that provides access to the "database"
fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}

/// A filter that provides a Person deserialized from JSON
fn json_body() -> impl Filter<Extract = (Person,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(MAX_BYTES)
        .and(warp::body::json())
}


#[derive(Debug)]
struct InvalidCSV;
impl warp::reject::Reject for InvalidCSV {}


pub fn csv_body() -> impl Filter<Extract = (Person,), Error = Rejection> + Copy {
    use warp::hyper::body::Bytes;
    
    warp::body::content_length_limit(MAX_BYTES)
        .and(warp::body::bytes())
        .and_then(|buf: Bytes| async move {

            let results = crate::io::parse_csv_people_from_reader(
                buf.reader(), ',', false);

            let person = results.into_iter()
                .next()
                .expect("Unable to deserialize record from CSV");

            person.map_err(|_| { warp::reject::custom(InvalidCSV) })
        })
}


pub fn records(db: Db)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    records_list(db.clone())
        .or(records_sorted_by_column(db.clone()))
        .or(create_record(db))
}


pub fn records_list(db: Db)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("records")
        .and(warp::get())
        .and(warp::query::<models::ListOptions>())
        .and(with_db(db.clone()))
        .and_then(handlers::list_records)
}


pub fn records_sorted_by_column(db: Db)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("records" / String)
        .and(warp::get())
        .and(warp::query::<models::ListOptions>())
        .and(with_db(db))
        .and_then(handlers::list_records_sorted_by_field)
}


pub fn create_record(db: Db)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    create_record_from_csv(db.clone())
        .or(create_record_from_json(db))
}


pub fn create_record_from_csv(db: Db)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path("records")
        .and(warp::post())
        .and(warp::header::exact_ignore_case("content-type", "text/csv"))
        .and(csv_body())
        .and(with_db(db))
        .and_then(handlers::create_record)
}


pub fn create_record_from_json(db: Db)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path("records")
        .and(warp::post())
        .and(warp::header::exact_ignore_case("content-type", "application/json"))
        .and(json_body())
        .and(with_db(db))
        .and_then(handlers::create_record)
}


mod tests;
