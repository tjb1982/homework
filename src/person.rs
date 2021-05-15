use std::{cmp::Ordering};
use log::{warn};
use serde::Deserialize;


#[derive(Debug, Clone, Eq, Ord, PartialEq, PartialOrd, Deserialize)]
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

    fn _cmp_order_by_attrs(a: &Self, b: &Self, attrs: &Vec<&str>, prev: Ordering) -> Ordering {
        if attrs.len() == 0 {
            return prev
        }
    
        match prev {
            Ordering::Equal => {            
                let rest = attrs[1..].to_vec();
                let attr = attrs[0];
            
                match Self::cmp_attr(a, b, attr) {
                    Ordering::Equal => Self::_cmp_order_by_attrs(a, b, &rest, prev),
                    x => x
                }
            },
            _ => prev
        }    
    }

    pub fn cmp_order_by_attrs(a: &Self, b: &Self, attrs: &Vec<&str>) -> Ordering {
        match attrs.len() {
            0 => a.cmp(b),
            _ => Self::_cmp_order_by_attrs(a, b, attrs, Ordering::Equal)
        }
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    fn create_person(first_name: &str, last_name: &str, email: &str, favorite_color: &str, dob: &str) -> Person {
        Person {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            email: String::from(email),
            favorite_color: String::from(favorite_color),
            dob: String::from(dob)
        }
    }

    fn create_people() -> [Person; 4] {
        [
            create_person("Tom","Brennan", "tjb1982@gmail.com", "red", "1982-08-19"),
            create_person("Rachel","Fuller", "tjb1982@gmail.com", "green", "1970-08-10"),
            create_person("Chester","Brennan", "", "", ""),
            create_person("June","Brennan", "", "", ""),
        ]
    }
    
    fn create_people_sorted_by_attrs(attrs: Vec<&str>) -> Vec<Person> {
        let mut people = create_people();

        people.sort_by(|a, b| Person::cmp_order_by_attrs(a, b, &attrs));
        people.to_vec()
    }
    
    #[test]
    fn empty_attr_sort() {
        let mut people1 = create_people();
        let mut people2 = create_people();

        people1.sort();
        people2.sort_by(
            |a, b| Person::cmp_order_by_attrs(a, b, &vec![])
        );

        for (idx, item) in people1.iter().enumerate() {
            assert!(item.first_name.eq(&people2[idx].first_name));
        }
        assert!(people2[0].first_name.eq("Chester"));
        assert!(people2[1].first_name.eq("June"));
        assert!(people2[2].first_name.eq("Tom"));
        assert!(people2[3].first_name.eq("Rachel"));
    }

    #[test]
    fn first_name_sort() {
        let people = create_people_sorted_by_attrs(vec!["first_name"]);
        
        assert!(people[0].first_name.eq("Chester"));
        assert!(people[1].first_name.eq("June"));
        assert!(people[2].first_name.eq("Rachel"));
        assert!(people[3].first_name.eq("Tom"));
    }

    #[test]
    fn last_name_sort() {
        let people = create_people_sorted_by_attrs(vec!["last_name"]);

        assert!(people[0].first_name.eq("Tom"));
        assert!(people[1].first_name.eq("Chester"));
        assert!(people[2].first_name.eq("June"));
        assert!(people[3].first_name.eq("Rachel"));
    }

    #[test]
    fn last_then_first_sort() {
        let people = create_people_sorted_by_attrs(vec!["last_name", "first_name"]);

        assert!(people[0].first_name.eq("Chester"));
        assert!(people[1].first_name.eq("June"));
        assert!(people[2].first_name.eq("Tom"));
        assert!(people[3].first_name.eq("Rachel"));
    }
}
