use std::thread;

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
        println!("{} is eating.", self.name);

        thread::sleep_ms(1000);

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

    let handles: Vec<_> = philosophers.into_iter().map(|p| {
        thread::spawn(move || {
            p.eat();
        })
    }).collect();


    for h in handles {
        h.join().unwrap();
    }
}
