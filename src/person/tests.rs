#![allow(dead_code)]

use super::*;

#[test]
fn new_person() {
    let person = Person::new(
        "Tom", "Brennan", "tjb1982@gmail.com", "red", "1982-08-19"
    );

    assert!(person.first_name.eq("Tom"));
    assert!(person.last_name.eq("Brennan"));
    assert!(person.email.eq("tjb1982@gmail.com"));
    assert!(person.favorite_color.eq("red"));
    assert!(person.dob.eq("1982-08-19"));
}


mod sorting {

    use super::*;

    fn create_people() -> [Person; 4] {
        [
            Person::new("Tom","Brennan", "tjb1982@gmail.com", "red", "1982-08-19"),
            Person::new("Rachel","Fuller", "tjb1982@gmail.com", "green", "1970-08-10"),
            Person::new("Chester","Brennan", "", "", ""),
            Person::new("June","Brennan", "", "", ""),
        ]
    }
    
    fn create_people_sorted_by_fields(fields: Vec<&str>) -> Vec<Person> {
        let mut people = create_people();

        people.sort_by(|a, b| Person::cmp_order_by_fields(a, b, &fields));
        people.to_vec()
    }

    
    #[test]
    fn empty_field_sort_mirrors_plain_sort() {
        let mut people1 = create_people();
        let mut people2 = create_people();

        people1.sort();
        people2.sort_by(
            |a, b| Person::cmp_order_by_fields(a, b, &vec![])
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
        let people = create_people_sorted_by_fields(vec!["first_name"]);
        
        assert!(people[0].first_name.eq("Chester"));
        assert!(people[1].first_name.eq("June"));
        assert!(people[2].first_name.eq("Rachel"));
        assert!(people[3].first_name.eq("Tom"));
    }

    #[test]
    fn last_name_sort() {
        let people = create_people_sorted_by_fields(vec!["last_name"]);

        assert!(people[0].first_name.eq("Tom"));
        assert!(people[1].first_name.eq("Chester"));
        assert!(people[2].first_name.eq("June"));
        assert!(people[3].first_name.eq("Rachel"));
    }

    #[test]
    fn last_then_first_sort() {
        let people = create_people_sorted_by_fields(vec!["last_name", "first_name"]);

        assert!(people[0].first_name.eq("Chester"));
        assert!(people[1].first_name.eq("June"));
        assert!(people[2].first_name.eq("Tom"));
        assert!(people[3].first_name.eq("Rachel"));
    }
}

mod struct_fields {
    use crate::struct_fields::StructFieldDeserialize;
    use super::*;

    #[test]
    fn get_struct_fields() {
        let fields = Person::struct_fields();
        
        assert!(fields.len() == 5);
        assert!(fields[0].eq("last_name"));
        assert!(fields[1].eq("first_name"));
        assert!(fields[2].eq("email"));
        assert!(fields[3].eq("favorite_color"));
        assert!(fields[4].eq("dob"));
    }
}