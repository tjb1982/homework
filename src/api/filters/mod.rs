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
/// Body must be under `MAX_BYTES` length.
fn json_body() -> impl Filter<Extract = (Person,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(MAX_BYTES)
        .and(warp::body::json())
}


#[derive(Debug)]
struct InvalidCSV;
impl warp::reject::Reject for InvalidCSV {}

/// Filter that provides a Person deserialized from CSV.
/// N.B. that the body should not be urlencoded.
/// Body must be under `MAX_BYTES` size.
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


/// Filter that combines all of the `records_` filters.
pub fn records(db: Db)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    records_list(db.clone())
        .or(records_sorted_by_column(db.clone()))
        .or(create_record(db))
}


/// Filter that responds with a list of records.
/// A query-string may be provided with the values encoded in `models::ListOptions`
/// which provides simple pagination.
pub fn records_list(db: Db)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("records")
        .and(warp::get())
        .and(warp::query::<models::ListOptions>())
        .and(with_db(db.clone()))
        .and_then(handlers::list_records)
}


/// Filter that responds with a list of records, similar to `records_list` (incl. pagination),
/// but also sorts the records ascending, according to the field provided in the path.
///
/// E.g., /records/last_name
///
/// reponds with a list of records ordered by `Persion.last_name` ascending.
pub fn records_sorted_by_column(db: Db)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    warp::path!("records" / String)
        .and(warp::get())
        .and(warp::query::<models::ListOptions>())
        .and(with_db(db))
        .and_then(handlers::list_records_sorted_by_field)
}


/// Filter that provides a mechanism for `POST`ing a record to the database.
/// There are two methods: CSV and JSON. This filter proxies to those.
pub fn create_record(db: Db)
    -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
{
    create_record_from_csv(db.clone())
        .or(create_record_from_json(db))
}


/// Filter that provides a POST endpoint for a body containing a single CSV row
/// representing a record.
/// Content-Type must be set to exactly `text/csv`. UTF-8 is assumed.
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


/// Filter that provides a POST endpoint for a body containing a single JSON object
/// representing a record.
/// Content-Type must be set to exactly `application/json`.
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