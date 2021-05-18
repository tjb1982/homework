use warp::Filter;

use crate::api::models::{self, Db};
use crate::api::handlers;
use crate::person::Person;


fn with_db(db: Db) -> impl Filter<Extract = (Db,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || db.clone())
}


fn json_body() -> impl Filter<Extract = (Person,), Error = warp::Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16)
        .and(warp::body::json())
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
    warp::path("records")
        .and(warp::post())
        .and(with_db(db))
        .and(json_body())
        .and_then(handlers::create_record)
}
