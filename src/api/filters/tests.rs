#![allow(dead_code)]

use super::*;
use crate::{serialization::{StructFieldDeserialize, date_format::str_from_date}};

const LAST_NAME: &str = "Brennan";
const FIRST_NAME: &str = "Tom";
const EMAIL: &str = "tjb1982@gmail.com";
const FAVORITE_COLOR: &str = "red";
const DOB: &str = "8/19/1982";


fn expected(this: &str, that: &str) -> String {
    format!("expected: \"{}\", got: \"{}\"", this, that)
}

fn assert_person_struct_fields(person: &Person) {
    for &field in Person::struct_fields().into_iter() {
        match field {
            "last_name" => assert!(
                person.last_name.eq(LAST_NAME), "{}", expected(LAST_NAME, person.last_name.as_str())
            ),
            "first_name" => assert!(
                person.first_name.eq(FIRST_NAME), "{}", expected(FIRST_NAME, person.first_name.as_str())
            ),
            "email" => assert!(
                person.email.eq(EMAIL), "{}", expected(EMAIL, person.email.as_str())
            ),
            "favorite_color" => assert!(
                person.favorite_color.eq(FAVORITE_COLOR), "{}", expected(FAVORITE_COLOR, person.favorite_color.as_str())
            ),
            "dob" => {
                let dob = str_from_date(&person.dob.unwrap());
                assert!(dob.eq(DOB), "{}", expected(DOB, person.last_name.as_str()))
            },
            x => assert!(false, "Field \"{}\" shouldn't exist.", x)
        }
    }
}


fn assert_eq_person(from_rs: &Person, from_db: &Person) {
    assert_eq!(
        *from_db, *from_rs,
        "Person from database ({:?}) didn't match person from result-set ({:?}).",
        *from_db, *from_rs
    );
}


#[cfg(test)]
mod get {
    use std::sync::Arc;
    use tokio::sync::Mutex;

    use super::*;
    
    fn init_db() -> Arc<Mutex<Vec<Person>>> {
        Arc::new(Mutex::new(vec![
            Person::new(LAST_NAME, FIRST_NAME, EMAIL, FAVORITE_COLOR, DOB),
            Person::new("Brennan", "Chester", "", "green", ""),
            Person::new("Fuller", "Rachel", "", "green", "8/10/1970"),
            Person::new("Brennan", "June", "", "", "")
        ]))
    }

    #[tokio::test]
    async fn get_records() {
        let db = init_db();
        let filter = records(db.clone());
        let response = warp::test::request()
            .path("/records")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), 200);

        let results = serde_json::from_slice::<Vec<Person>>(response.body());
        let db_people = db.lock().await;

        match results {
            Ok(people) => {
                assert_eq!(people.len(), db_people.len());
                for (idx, person) in db_people.iter().enumerate() {
                    let (from_db, from_rs) = (person, &people[idx]);
                    assert_eq_person(from_rs, from_db);
                }
            },
            Err(e) => assert!(false, "{:?}", e)
        }
    }


    #[tokio::test]
    async fn get_records_sorted_by_bad_column() {
        let db = init_db();
        let filter = records_sorted_by_column(db.clone());
        let response = warp::test::request()
            .path("/records/foo")
            .reply(&filter)
            .await;

        assert_eq!(response.status(), 404);

        let result = serde_json::from_slice::<handlers::APIError>(response.body());

        match result {
            Ok(api_error) => {
                assert!(api_error.reason.eq("Not Found"), "Bad `reason`: {}", api_error.reason);
                assert!(api_error.context.eq(&format!("Available fields: {}", &Person::struct_fields().join(", "))),
                    "Bad `context`: {}", api_error.context);
            },
            Err(e) => assert!(false, "{:?}", e)
        }    
    }


    async fn get_records_sorted_by_column(column: &'static str, first_idx: usize, last_idx: usize) {
        let db = init_db();
        let filter = records_sorted_by_column(db.clone());
        let response = warp::test::request()
            .path(format!("/records/{}", column).as_str())
            .reply(&filter)
            .await;

        assert_eq!(response.status(), 200);

        let results = serde_json::from_slice::<Vec<Person>>(response.body());
        let db_people = db.lock().await;

        match results {
            Ok(people) => {
                let first = people.first().unwrap();
                let last = people.last().unwrap();

                assert_eq_person(first, &db_people[first_idx]);
                assert_eq_person(last, &db_people[last_idx]);
            },
            Err(e) => assert!(false, "{:?}", e)
        }
    }


    #[tokio::test]
    async fn get_records_sorted_by_last_name() {
        let _ = get_records_sorted_by_column("last_name", 0, 2);
    }

    #[tokio::test]
    async fn get_records_sorted_by_first_name() {
        let _ = get_records_sorted_by_column("first_name", 1, 0);
    }

    #[tokio::test]
    async fn get_records_sorted_by_email() {
        let _ = get_records_sorted_by_column("email", 0, 3);
    }

    #[tokio::test]
    async fn get_records_sorted_by_favorite_color() {
        let _ = get_records_sorted_by_column("favorite_color", 1, 3);
    }

    #[tokio::test]
    async fn get_records_sorted_by_dob() {
        let _ = get_records_sorted_by_column("dob", 2, 3);
    }

}

#[cfg(test)]
mod post {

    use super::*;
    use std::{cmp::Ordering};


    fn assert_missing_field(e: Rejection) {
        assert!("missing field".cmp(format!("{:?}", e).as_str()) == Ordering::Greater)
    }


    #[tokio::test]
    async fn post_json() {

        let request = warp::test::request().body(format!(r###"{{
                "last_name": "{}",
                "first_name": "{}",
                "email": "{}",
                "favorite_color": "{}",
                "dob": "{}"
            }}"###, LAST_NAME, FIRST_NAME, EMAIL, FAVORITE_COLOR, DOB));

        let person = request.filter(&json_body()).await.unwrap();

        assert_person_struct_fields(&person)
    }


    #[tokio::test]
    async fn post_json_missing_fields() {

        let request = warp::test::request().body(r###"{
                "last_name": "foo"
            }"###);

        match request.filter(&json_body()).await {
            Ok(x) => assert!(false, "{:?} should not exist.", x),
            Err(e) => assert_missing_field(e)
        }
    }


    #[tokio::test]
    async fn post_json_additional_field() {

        let request = warp::test::request().body(format!(r###"{{
                "last_name": "{}",
                "first_name": "{}",
                "email": "{}",
                "favorite_color": "{}",
                "dob": "{}",
                "foo": "bar"
            }}"###, LAST_NAME, FIRST_NAME, EMAIL, FAVORITE_COLOR, DOB));

        match request.filter(&json_body()).await {
            Ok(p) => assert_person_struct_fields(&p),
            Err(e) => assert!(false, "{:?}", e)
        }
    }


    #[tokio::test]
    async fn post_csv() {
        let request = warp::test::request().body(
            format!(r###"{}, {}, {}, {}, {}
                "###, LAST_NAME, FIRST_NAME, EMAIL, FAVORITE_COLOR, DOB));

        match request.filter(&csv_body()).await {
            Ok(p) => assert_person_struct_fields(&p),
            Err(e) => assert!(false, "{:?}", e)
        }
    }


    #[tokio::test]
    async fn post_csv_missing_fields() {
        let request = warp::test::request().body(
            "Brennan,
                "
        );

        match request.filter(&csv_body()).await {
            Ok(p) => assert!(false, "Should not exist: {:?}", p),
            Err(e) => assert_missing_field(e)
        }
    }


    #[tokio::test]
    async fn post_csv_additional_fields() {
        let request = warp::test::request().body(
            format!("{}, {}, {}, {}, {}, foo",
                LAST_NAME, FIRST_NAME, EMAIL, FAVORITE_COLOR, DOB)
        );

        match request.filter(&csv_body()).await {
            Ok(p) => assert_person_struct_fields(&p),
            Err(e) => assert!(false, "{:?}", e)
        }
    }
}