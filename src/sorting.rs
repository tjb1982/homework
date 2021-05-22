use std::{cmp::Ordering, convert::Infallible, str::FromStr, string::ParseError};
use serde::Deserialize;


#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "lowercase")]
pub enum SortDirection {
    Asc,
    Desc
}


/// Since this trait is private, the method declared here will not be available
/// outside of this module, thereby making it private. But in order for it to be
/// useful within the `FieldsOrd` trait, we have to define an implementation for
/// any type that also implements `FieldsOrd`. Then the compiler knows that any
/// `FieldsOrd` also has the `FieldsOrdImpl` trait.
trait FieldsOrdImpl {
    fn _cmp_order_by_fields_impl(&self, b: &Self, fields: &Vec<(&str, SortDirection)>, prev: Ordering) -> Ordering;
}


pub trait FieldsOrd: Eq + Ord + Sized {

    fn cmp_field(&self, b: &Self, field: &str, direction: &SortDirection) -> Ordering;

    fn cmp_order_by_fields(&self, b: &Self, fields: &Vec<(&str, SortDirection)>) -> Ordering
    {
        match fields.len() {
            0 => self.cmp(b),
            _ => self._cmp_order_by_fields_impl(b, fields, Ordering::Equal)
        }
    }
}


impl<T: FieldsOrd> FieldsOrdImpl for T {
    fn _cmp_order_by_fields_impl(&self, b: &Self, fields: &Vec<(&str, SortDirection)>, prev: Ordering) -> Ordering
    {
        if fields.len() == 0 {
            return prev
        }

        match prev {
            Ordering::Equal => {
                let rest = fields[1..].to_vec();
                let (field, direction) = &fields[0];
            
                match self.cmp_field(b, field, &direction) {
                    Ordering::Equal => self._cmp_order_by_fields_impl(b, &rest, prev),
                    x => x
                }
            },
            _ => prev
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
