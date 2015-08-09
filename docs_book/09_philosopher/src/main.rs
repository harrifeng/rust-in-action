struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }

    fn eat(&self) {
        println!("{} is done eating.", self.name);
    }

}


fn main() {
    let philosophers = vec![
        Philosopher::new("Judith Bulter 1"),
        Philosopher::new("Judith Bulter 2"),
        Philosopher::new("Judith Bulter 3"),
        Philosopher::new("Judith Bulter 4"),
        Philosopher::new("Judith Bulter 5"),
        ];

    for p in &philosophers {
        p.eat();
    }
}
