use std::convert::Infallible;
use warp::{Rejection, Reply, hyper::StatusCode, reject, reply::with_status};
use serde::Serialize;

use crate::{api::models::{ListOptions, Db}, serialization::StructFieldDeserialize, sort_direction::SortDirection};
use crate::person::Person;


const MAX_PER_PAGE: usize = 50;


fn pagination(opts: &ListOptions) -> (usize, usize)
{
    let per_page = opts.per_page.unwrap_or(MAX_PER_PAGE);
    let page = opts.page.unwrap_or(1) - 1;

    (page * per_page, per_page)
}


pub async fn list_records(opts: ListOptions, db: Db)
    -> Result<impl warp::Reply, Infallible>
{
    let (offset, limit) = pagination(&opts);

    let people: Vec<Person> = db.lock().await.clone()
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();
    Ok(warp::reply::json(&people))
}


#[derive(Serialize)]
struct APIError {
    reason: &'static str,
    context: String,
}


pub fn not_found(context: String) -> Result<warp::reply::WithStatus<warp::reply::Json>, Infallible>
{
    let status = StatusCode::NOT_FOUND;
    let err = APIError {
        reason: status.canonical_reason().unwrap(),
        context,
    };

    Ok(with_status(warp::reply::json(&err), status))
}


pub async fn list_records_sorted_by_field(field: String, opts: ListOptions, db: Db)
    -> Result<warp::reply::WithStatus<warp::reply::Json>, Infallible>
{

    let person_fields = Person::struct_fields();
    if !person_fields.contains(&field.as_str()) {
        return not_found(format!("Available fields: {}", person_fields.join(", ")));
    }

    let (offset, limit) = pagination(&opts);

    let fields = vec![
        (field.as_str(), opts.direction.unwrap_or(SortDirection::Asc))
    ];

    let mut people: Vec<Person> = db.lock().await.clone();

    people.sort_by(|a, b| Person::cmp_order_by_fields(a, b, &fields));

    let sorted: Vec<Person> = people
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    Ok(with_status(warp::reply::json(&sorted), StatusCode::OK))
}


pub async fn create_record(db: Db, record: Person)
    -> Result<impl warp::Reply, Infallible>
{
    let mut people = db.lock().await;
    people.push(record);

    Ok(StatusCode::CREATED)
}