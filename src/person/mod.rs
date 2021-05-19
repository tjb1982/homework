use std::{cmp::Ordering};
use log::{warn};
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

use crate::serialization::{self, date_format};
use crate::sort_direction::SortDirection;


#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Person {
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub favorite_color: String,

    #[serde(with = "date_format")]
    pub dob: Option<NaiveDate>,
}


impl serialization::StructFieldDeserialize for Person {
    fn struct_fields() -> &'static[&'static str] {
        let mut fields = None;
        
        let _ = Self::deserialize(serialization::StructFieldsDeserializer {
            fields: &mut fields
        });

        fields.unwrap()
    }
}


impl Person {

    pub fn new(last_name: &str, first_name: &str, email: &str, favorite_color: &str, dob: &str) -> Person {
        Person {
            last_name: String::from(last_name),
            first_name: String::from(first_name),
            email: String::from(email),
            favorite_color: String::from(favorite_color),
            dob: match date_format::date_from_str(&dob.to_string()) {
                Ok(dob) => Some(dob),
                _ => None
            }
        }
    }


    fn cmp_field(a: &Self, b: &Self, field: &str, direction: &SortDirection) -> Ordering {
        let ord = match field {
            "first_name" => a.first_name.cmp(&b.first_name),
            "last_name" => a.last_name.cmp(&b.last_name),
            "email" => a.email.cmp(&b.email),
            "favorite_color" => a.favorite_color.cmp(&b.favorite_color),
            "dob" => a.dob.cmp(&b.dob),
            _ => {
                warn!("Field \"{}\" not found: ignoring.", field);
                Ordering::Equal
            }
        };

        match direction {
            SortDirection::Desc => ord.reverse(),
            _ => ord
        }
    }


    fn cmp_order_by_fields_impl(a: &Self, b: &Self, fields: &Vec<(&str, SortDirection)>, prev: Ordering) -> Ordering {
        if fields.len() == 0 {
            return prev
        }
    
        match prev {
            Ordering::Equal => {
                let rest = fields[1..].to_vec();
                let (field, direction) = &fields[0];
            
                match Self::cmp_field(a, b, field, &direction) {
                    Ordering::Equal => Self::cmp_order_by_fields_impl(a, b, &rest, prev),
                    x => x
                }
            },
            _ => prev
        }    
    }

    pub fn cmp_order_by_fields(a: &Self, b: &Self, fields: &Vec<(&str, SortDirection)>) -> Ordering {
        match fields.len() {
            0 => a.cmp(b),
            _ => Self::cmp_order_by_fields_impl(a, b, fields, Ordering::Equal)
        }
    }
}

mod tests;
