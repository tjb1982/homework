#![allow(dead_code)]

use super::*;


#[test]
fn new_person() {
    let first_name = "Tom";
    let last_name = "Brennan";
    let email = "tjb1982@gmail.com";
    let favorite_color = "red";
    let dob = "8/19/1982";

    let person = Person::new(last_name, first_name, email, favorite_color, dob);

    assert!(person.first_name.eq(first_name));
    assert!(person.last_name.eq(last_name));
    assert!(person.email.eq(email));
    assert!(person.favorite_color.eq(favorite_color));
    assert!(person.dob.eq(&Some(date_format::date_from_str(&dob.to_string()).unwrap())));
}


mod sorting {

    use super::*;

    fn create_people() -> [Person; 4] {
        [
            Person::new("Brennan","Tom", "tjb1982@gmail.com", "red", "8/19/1982"),
            Person::new("Fuller","Rachel", "tjb1982@gmail.com", "green", "8/10/1970"),
            Person::new("Brennan","Chester", "", "", ""),
            Person::new("Brennan","June", "", "", ""),
        ]
    }
    
    fn create_people_sorted_by_fields(fields: Vec<(&str, SortDirection)>) -> Vec<Person> {
        let mut people = create_people();

        people.sort_by(|a, b| a.cmp_order_by_fields(b, &fields));
        people.to_vec()
    }

    
    #[test]
    fn empty_field_sort_mirrors_plain_sort() {
        let mut people1 = create_people();
        let mut people2 = create_people();

        people1.sort();
        people2.sort_by(
            |a, b| a.cmp_order_by_fields(b, &vec![])
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
        let people = create_people_sorted_by_fields(vec![
            ("first_name", SortDirection::Asc)
        ]);
        
        assert!(people[0].first_name.eq("Chester"));
        assert!(people[1].first_name.eq("June"));
        assert!(people[2].first_name.eq("Rachel"));
        assert!(people[3].first_name.eq("Tom"));
    }

    #[test]
    fn last_name_sort() {
        let people = create_people_sorted_by_fields(vec![
            ("last_name", SortDirection::Asc)
        ]);

        assert!(people[0].first_name.eq("Tom"));
        assert!(people[1].first_name.eq("Chester"));
        assert!(people[2].first_name.eq("June"));
        assert!(people[3].first_name.eq("Rachel"));
    }

    #[test]
    fn last_then_first_sort() {
        let people = create_people_sorted_by_fields(vec![
            ("last_name", SortDirection::Asc),
            ("first_name", SortDirection::Desc)
        ]);

        assert!(people[0].first_name.eq("Tom"));
        assert!(people[1].first_name.eq("June"));
        assert!(people[2].first_name.eq("Chester"));
        assert!(people[3].first_name.eq("Rachel"));
    }
}

#[allow(unused_imports)]
mod struct_fields {
    use crate::serialization::StructFieldDeserialize;
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
