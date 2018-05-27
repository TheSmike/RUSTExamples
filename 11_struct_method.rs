fn main(){
	let duck = Duck { name: String::from("Donald") };
	duck.quack();
	duck.jump(0.9);
}

struct Duck{ name: String }

impl Duck {
    fn quack(&self) {
        println!("{} say quack!", self.name);
    }

	fn jump(&self, height : f64) {
        println!("{} jump up {} meters", self.name, height);
    }
}
