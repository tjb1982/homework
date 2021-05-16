use std::{cmp::Ordering};
use log::{warn};
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Person {
    last_name: String,
    first_name: String,
    email: String,
    favorite_color: String,
    dob: String,
}


impl Person {

    fn cmp_attr(a: &Self, b: &Self, attr: &str) -> Ordering {
        match attr {
            "first_name" => a.first_name.cmp(&b.first_name),
            "last_name" => a.last_name.cmp(&b.last_name),
            "email" => a.email.cmp(&b.email),
            "favorite_color" => a.favorite_color.cmp(&b.favorite_color),
            "dob" => a.dob.cmp(&b.dob),
            _ => {
                warn!("attr \"{}\" not found: ignoring.", attr);
                Ordering::Equal
            }
        }
    }    

    fn cmp_order_by_attrs_impl(a: &Self, b: &Self, attrs: &Vec<&str>, prev: Ordering) -> Ordering {
        if attrs.len() == 0 {
            return prev
        }
    
        match prev {
            Ordering::Equal => {            
                let rest = attrs[1..].to_vec();
                let attr = attrs[0];
            
                match Self::cmp_attr(a, b, attr) {
                    Ordering::Equal => Self::cmp_order_by_attrs_impl(a, b, &rest, prev),
                    x => x
                }
            },
            _ => prev
        }    
    }

    pub fn cmp_order_by_attrs(a: &Self, b: &Self, attrs: &Vec<&str>) -> Ordering {
        match attrs.len() {
            0 => a.cmp(b),
            _ => Self::cmp_order_by_attrs_impl(a, b, attrs, Ordering::Equal)
        }
    }
}


mod tests;
