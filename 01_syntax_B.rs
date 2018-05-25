
struct User {
	username: String,
	email: String,
	sign_in_count: u64,
	active: bool,
}

fn main() {
	let user1 = User {
		email: String::from("michele.scarpenti@studenti.unipr.it"),
		username: String::from("micheleS"),
		active: true,
		sign_in_count: 1,
	};

println!("username: {}", user1.username);
}

fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn build_user_from_another(user1 : User) -> User{
	let user2 = User {
		email: String::from("james.mk@gmail.com"),
		username: String::from("jmk"),
		..user1
	};

	 user2
}
