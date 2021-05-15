use std::io;
use csv;

mod person;
use person::Person;


fn main() -> io::Result<()> {
    let file = std::fs::File::open("test.csv")?;
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(false)
        .trim(csv::Trim::All)
        .from_reader(file);

    let attrs = vec!["last_name", "first_name"];

    let mut people: Vec<Person> = reader.deserialize::<Person>().map(
        |result| match result {
            Ok(person) => person,
            Err(e) => panic!("Corrupt CSV file: {}", e)
        }
    ).collect();

    people.sort_by(|a, b| Person::cmp_order_by_attrs(a, b, &attrs));

    println!("{:?}", people);

    Ok(())
}
