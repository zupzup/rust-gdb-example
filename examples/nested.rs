use rust_gdb_example::*;

fn main() {
    let animals: Vec<Animal> = vec![
        Animal {
            kind: AnimalType::Cat,
            name: "Chip".to_string(),
            age: 4,
        },
        Animal {
            kind: AnimalType::Cat,
            name: "Nacho".to_string(),
            age: 6,
        },
        Animal {
            kind: AnimalType::Dog,
            name: "Taco".to_string(),
            age: 2,
        },
    ];

    let mut some_person = Person {
        name: "Some".to_string(),
        pets: animals,
        age: 24,
    };
    println!("person: {:?}", some_person);
    some_person.age = 100;
    some_person.name = some_func(&some_person.name);
}

fn some_func(name: &str) -> String {
    name.chars().rev().collect()
}
