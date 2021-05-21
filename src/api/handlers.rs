use std::convert::Infallible;
use warp::{Rejection, Reply, hyper::StatusCode, reply::with_status};
use serde::{Serialize, Deserialize};

use crate::{api::models::{ListOptions, Db}, sorting::SortDirection};
use crate::person::Person;
use crate::sorting::FieldsOrd;


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

    let mut people = db.lock().await.clone();
    
    people.sort_by(|a, b| a.cmp_order_by_fields(b, &vec![]));

    let people: Vec<Person> = people
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();
    
    Ok(warp::reply::json(&people))
}


#[derive(Debug, Serialize, Deserialize)]
pub struct APIError {
    pub reason: String,
    pub context: String,
}


pub fn not_found(context: String) -> Result<impl Reply, Infallible>
{
    let status = StatusCode::NOT_FOUND;
    let err = APIError {
        reason: status.canonical_reason().unwrap().to_string(),
        context,
    };

    Ok(with_status(warp::reply::json(&err), status))
}


pub async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible>
{
    use crate::api::filters;

    let reason: String;
    let code;
    let mut context = String::from("(None)");

    if err.is_not_found() {
        code = StatusCode::NOT_FOUND;
        reason = "Not found".into();
    } else if let Some(filters::InvalidCSV) = err.find() {
        code = StatusCode::BAD_REQUEST;
        reason = "Unable to parse CSV body".into();
    } else if let Some(filters::InvalidFilterField { available}) = err.find() {
        code = StatusCode::NOT_FOUND;
        reason = "Field not found".into();
        context = format!("Available fields: {}", available.join(", "));
    } else {
        // reason = "Unknown".into();
        reason = format!("{:?}", err);
        code = StatusCode::INTERNAL_SERVER_ERROR;
    }

    let json = warp::reply::json(&APIError {
        reason, context
    });

    Ok(with_status(json, code))
}


pub async fn list_records_sorted_by_field(field: String, opts: ListOptions, db: Db)
    -> Result<impl Reply, Infallible>
{
    let (offset, limit) = pagination(&opts);

    let fields = vec![
        (field.as_str(), opts.direction.unwrap_or(SortDirection::Asc))
    ];

    let mut people: Vec<Person> = db.lock().await.clone();

    people.sort_by(|a, b| a.cmp_order_by_fields(b, &fields));

    let sorted: Vec<&Person> = people
        .iter()
        .skip(offset)
        .take(limit)
        .collect();

    Ok(with_status(warp::reply::json(&sorted), StatusCode::OK))    
}


pub async fn create_record(record: Person, db: Db)
    -> Result<impl Reply, Infallible>
{
    let mut people = db.lock().await;
    people.push(record);

    Ok(StatusCode::CREATED)
}