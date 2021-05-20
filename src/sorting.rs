use std::{cmp::Ordering, convert::Infallible, str::FromStr, string::ParseError};
use serde::Deserialize;


#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc
}


pub trait FieldsSort: Eq + PartialEq + PartialOrd + Ord {

    fn cmp_field(&self, b: &Self, field: &str, direction: &SortDirection) -> Ordering;

    fn cmp_order_by_fields(&self, b: &Self, fields: &Vec<(&str, SortDirection)>) -> Ordering
    {
        match fields.len() {
            0 => self.cmp(b),
            _ => cmp_order_by_fields(self, b, fields, Ordering::Equal)
        }
    }
}


impl FromStr for SortDirection {
    type Err = ParseError;

    fn from_str(direction: &str) -> Result<Self, Infallible> {
        match direction {
            "asc" => Ok(SortDirection::Asc),
            "desc" => Ok(SortDirection::Desc),
            _ => {
                log::warn!("Unable to parse sort direction \"{}.\" Falling back to \"asc.\"", direction);
                Ok(SortDirection::Asc)
            },
        }
    }
}


fn cmp_order_by_fields<T>(a: &T, b: &T, fields: &Vec<(&str, SortDirection)>, prev: Ordering) -> Ordering
    where T: FieldsSort + ?Sized
{
    if fields.len() == 0 {
        return prev
    }

    match prev {
        Ordering::Equal => {
            let rest = fields[1..].to_vec();
            let (field, direction) = &fields[0];
        
            match a.cmp_field(b, field, &direction) {
                Ordering::Equal => cmp_order_by_fields(a, b, &rest, prev),
                x => x
            }
        },
        _ => prev
    }    
}
