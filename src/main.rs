struct Philosopher {
    name: String,
}
impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
}

fn main() {
    let p1 = Philosopher::new("Джудит Батлер");
    let p2 = Philosopher::new("Рая Дунаевская");
    let p3 = Philosopher::new("Зарубина Наталья");
    let p4 = Philosopher::new("Эмма Гольдман");
    let p5 = Philosopher::new("Анна Шмидт");
}