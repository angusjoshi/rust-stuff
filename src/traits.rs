trait Printable {
    fn print(&self);
}

struct Person {
    name: String,
    age: u32
}

impl Printable for Person {
    fn print(&self) {
        println!("{0} is {1} years old.", self.name, self.age);
    }
}

fn print_thing(thing: &impl Printable) {
    thing.print();
}

#[cfg(test)]
mod tests {
    use crate::traits;
    use traits::Person;
    use crate::traits::print_thing;

    #[test]
    fn test_person_print() {
        let bob = Person {
            name: String::from("Bob"),
            age: 45
        };

        print_thing(&bob);

        assert![true]
    }
}