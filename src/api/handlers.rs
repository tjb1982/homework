use std::convert::Infallible;
use warp::{Rejection, Reply, hyper::StatusCode, reply::with_status};
use serde::{Serialize, Deserialize};

use crate::{api::models::{ListOptions, Db}, sorting::SortDirection};
use crate::person::Person;
use crate::sorting::FieldsOrd;


const MAX_PER_PAGE: usize = 50;


#[derive(Serialize, Deserialize)]
pub struct ResultSet {
    curr: usize,
    next: Option<usize>,
    prev: Option<usize>,
    first: usize,
    last: usize,
    pub count: usize,
    pub length: usize,
    pub results: Vec<Person>,
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


fn pagination(opts: &ListOptions) -> (usize, usize, usize)
{
    let per_page = opts.per_page.unwrap_or(MAX_PER_PAGE);
    let page = opts.page.unwrap_or(1);
    let idx = page - 1;

    (page, idx * per_page, per_page)
}


pub fn resultset(people: Vec<Person>, opts: ListOptions) -> ResultSet
{
    let (curr, offset, limit) = pagination(&opts);

    let count = people.len();

    let subset: Vec<Person> = people
        .into_iter()
        .skip(offset)
        .take(limit)
        .collect();

    let last = count / limit;
    let next = if curr < last { Some(curr + 1) } else { None };
    let prev = if curr > 1 { Some(curr - 1) } else { None };

    ResultSet {
        curr,
        first: 1,
        last,
        next,
        prev,
        count: count,
        length: subset.len(),
        results: subset,
    }
}


pub async fn list_records(opts: ListOptions, db: Db)
    -> Result<impl Reply, Infallible>
{
    let mut people = db.lock().await.clone();
    
    // people.sort_by(|a, b| a.cmp_order_by_fields(b, &vec![]));
    people.sort();

    Ok(warp::reply::json(&resultset(people, opts)))
}


pub async fn list_records_sorted_by_field(field: String, opts: ListOptions, db: Db)
    -> Result<impl Reply, Infallible>
{
    let fields = vec![
        (field.as_str(), opts.direction.unwrap_or(SortDirection::Asc))
    ];

    let mut people: Vec<Person> = db.lock().await.clone();

    people.sort_by(|a, b| a.cmp_order_by_fields(b, &fields));

    Ok(warp::reply::json(&resultset(people, opts)))
}


pub async fn create_record(record: Person, db: Db)
    -> Result<impl Reply, Infallible>
{
    let mut people = db.lock().await;
    people.push(record);

    Ok(StatusCode::CREATED)
}