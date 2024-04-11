use std::fs::{File, OpenOptions};
use std::io::{self, prelude::*};
use std::path::Path;
use std::str::FromStr;
use std::u32;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Person {
    id: u32,
    name: String,
    age: u8
}

fn main() {

    let path = Path::new("people.json");
    let mut people_json = String::new();
    
    let mut file = match OpenOptions::new().write(true).read(true).open(path) {
        Ok(file) => file,
        Err(_error) => match File::create(path) {
            Ok(file) => file,
            Err(_) => panic!("Error when creating file")
        }
    };

    match file.read_to_string(&mut people_json) {
        Ok(_json) => {},
        Err(_error) => panic!("Error when reading file")
    };

    let mut persons: Vec<Person> = match serde_json::from_str(&mut people_json) {
        Ok(persons) => persons,
        Err(_error) => vec![]
    };
    
    let mut id = "".to_string();
    let mut name = "".to_string();
    let mut age = "".to_string();

    let mut real_id: u32;
    let real_age: u8;

    loop {
        println!("Please provide the following data:");
        println!("Person's id: ");

        real_id = match validate_field::<u32>(&mut id) {
            Ok(_id) => _id,
            Err(_) => continue
        };

        println!("Person's name: ");

        match validate_field::<String>(&mut name) {
            Ok(_name) => {},
            Err(_) => continue
        }

        println!("Person's age: ");

        real_age = match validate_field::<u8>(&mut age) {
            Ok(_age) => _age,
            Err(_) => continue
        };

        let person = Person {
            id: real_id,
            name: name.trim().to_string(),
            age: real_age
        };

        persons.push(person);

        let people_json = match serde_json::to_string::<Vec<Person>>(&persons) {
            Ok(json) => json,
            Err(error) => panic!("{}", error)
        };

        match file.rewind() {
            Ok(_s) => match file.write_all(people_json.as_bytes()) {
                Ok(_s) => println!("Person saved successfully!"),
                Err(error) => panic!("{}", error)
            },
            Err(error) => panic!("{}", error)
        }

        break;
    }

}

fn validate_field<T>(
    field: &mut String
) -> Result<T, ()> where T: FromStr {
    match io::stdin().read_line(field) {
        Err(error) => {
            println!("Error: {}", error);
            return Err(());
        },
        Ok(_field) => {
            match field.trim().parse::<T>() {
                Ok(_field) => return Ok(_field),
                Err(_) => {
                    println!("Please provide a valid value!");
                    return Err(());
                }
            };
        }
    }
}