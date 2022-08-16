use std::io::stdin;

struct Visitor {
    name: String,
    greeting: String
}

impl Visitor {
    fn new(name: &str, greeting: &str) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string()
        }
    }
    
    fn greet(&self) {
        println!("{}", self.greeting)
    }
}

fn main() {
    println!("Hi, what's your name?");
    let name = get_user_name();
    let club_members = [
        Visitor::new("bert", "Hello, Bert."),
        Visitor::new("carl", "Wassup, Carl!"),
        Visitor::new("hugo", "Who invited Hugo?")
    ];
    
    let invited =
        club_members
            .iter()
            .find(|member| member.name == name);
    
    match invited {
        Some(member) => member.greet(),
        None => println!("Get lost! You are not invited.")
    }
    
    // let has_invitation = check_invitation(club_members, &name);
    //
    //
    // if has_invitation {
    //     println!("Welcome, {}", name);
    // } else {
    //     println!("Get lost! You are not invited.");
    //     // list_club_members(club_members);
    // }
}

fn get_user_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
        .trim()
        .to_lowercase()
}

// fn list_club_members(list: [&str;3]) {
//     println!("Members are:");
//     for i in 0..list.len() {
//         println!("{}", list[i]);
//     }
// }

// fn check_invitation(list: [Visitor;3], name: &String) -> bool {
//     match list
//         .iter()
//         .find(|member| member.name == String::from(name)) {
//         Some(member) => member.greet(),
//         None => false
//     }
// }
