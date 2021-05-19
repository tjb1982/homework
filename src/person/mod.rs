use std::{cmp::Ordering};
use log::{warn};
use serde::{Serialize, Deserialize};
use chrono::NaiveDate;

use crate::serialization::{self, date_format};
use crate::sort_direction::SortDirection;


/// `struct` representing a "record"
#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct Person {
    pub last_name: String,
    pub first_name: String,
    pub email: String,
    pub favorite_color: String,

    #[serde(with = "date_format")]
    pub dob: Option<NaiveDate>,
}


/// Allows for the listing of a `Person`'s fields via serde
/// See `src/serialization.rs` for more details.
impl serialization::StructFieldDeserialize for Person {

    /// Returns a list of strings representing the fields
    /// of a `Person`
    fn struct_fields() -> &'static[&'static str] {
        let mut fields = None;
        
        let _ = Self::deserialize(serialization::StructFieldsDeserializer {
            fields: &mut fields
        });

        fields.unwrap()
    }
}


impl Person {

    /// Convenience method for creating a `Person` from `&str` components.
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


    fn cmp_field(&self, b: &Self, field: &str, direction: &SortDirection) -> Ordering {
        let ord = match field {
            "first_name" => self.first_name.cmp(&b.first_name),
            "last_name" => self.last_name.cmp(&b.last_name),
            "email" => self.email.cmp(&b.email),
            "favorite_color" => self.favorite_color.cmp(&b.favorite_color),
            "dob" => self.dob.cmp(&b.dob),
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


    fn cmp_order_by_fields_impl(&self, b: &Self, fields: &Vec<(&str, SortDirection)>, prev: Ordering) -> Ordering {
        if fields.len() == 0 {
            return prev
        }
    
        match prev {
            Ordering::Equal => {
                let rest = fields[1..].to_vec();
                let (field, direction) = &fields[0];
            
                match self.cmp_field(b, field, &direction) {
                    Ordering::Equal => self.cmp_order_by_fields_impl(b, &rest, prev),
                    x => x
                }
            },
            _ => prev
        }    
    }


    /// Comparator method intended to be passed to `Vec::sort_by`, e.g.,
    /// ```
    /// use homework::sort_direction::SortDirection;
    /// use homework::person::Person;
    ///
    /// let mut people = vec![Person::new("Smith", "John", "jsmith@example.com", "chartreuse", "1/1/1900")];
    /// let ref fields = vec![("favorite_color", SortDirection::Asc), ("last_name", SortDirection::Desc)];
    ///
    /// people.sort_by(|a, b| a.cmp_order_by_fields(b, fields));
    /// ```
    ///
    pub fn cmp_order_by_fields(&self, b: &Self, fields: &Vec<(&str, SortDirection)>) -> Ordering {
        match fields.len() {
            0 => self.cmp(b),
            _ => self.cmp_order_by_fields_impl(b, fields, Ordering::Equal)
        }
    }
}

mod tests;
