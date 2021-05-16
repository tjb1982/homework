use std::{cmp::Ordering};
use log::{warn};
use serde::{Serialize, Deserialize};

use crate::struct_fields;

#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Person {
    last_name: String,
    first_name: String,
    email: String,
    favorite_color: String,
    dob: String,
}


impl struct_fields::StructFieldDeserialize for Person {
    fn struct_fields() -> &'static[&'static str] {
        let mut fields = None;
        
        match Self::deserialize(struct_fields::StructFieldsDeserializer { fields: &mut fields }) {
            Err(e) => log::error!("Problem deserializing fields for Person: {}", e),
            _ => ()
        };

        fields.unwrap()
    }
}


impl Person {

    fn new(first_name: &str, last_name: &str, email: &str, favorite_color: &str, dob: &str) -> Person {
        Person {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            email: String::from(email),
            favorite_color: String::from(favorite_color),
            dob: String::from(dob)
        }
    }


    fn cmp_field(a: &Self, b: &Self, field: &str) -> Ordering {
        match field {
            "first_name" => a.first_name.cmp(&b.first_name),
            "last_name" => a.last_name.cmp(&b.last_name),
            "email" => a.email.cmp(&b.email),
            "favorite_color" => a.favorite_color.cmp(&b.favorite_color),
            "dob" => a.dob.cmp(&b.dob),
            _ => {
                warn!("Field \"{}\" not found: ignoring.", field);
                Ordering::Equal
            }
        }
    }    

    fn cmp_order_by_fields_impl(a: &Self, b: &Self, fields: &Vec<&str>, prev: Ordering) -> Ordering {
        if fields.len() == 0 {
            return prev
        }
    
        match prev {
            Ordering::Equal => {            
                let rest = fields[1..].to_vec();
                let field = fields[0];
            
                match Self::cmp_field(a, b, field) {
                    Ordering::Equal => Self::cmp_order_by_fields_impl(a, b, &rest, prev),
                    x => x
                }
            },
            _ => prev
        }    
    }

    pub fn cmp_order_by_fields(a: &Self, b: &Self, fields: &Vec<&str>) -> Ordering {
        match fields.len() {
            0 => a.cmp(b),
            _ => Self::cmp_order_by_fields_impl(a, b, fields, Ordering::Equal)
        }
    }
}

mod tests;
