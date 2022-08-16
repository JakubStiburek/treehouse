use std::io::stdin;

#[derive(Debug)]
struct Visitor {
    name: String,
    action: VisitorAction,
    age: i8,
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }
    
    fn greet(&self) {
        match &self.action {
            VisitorAction::Accept => println!("Welcome, {}!", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome, {}!", self.name);
                println!("{}", note);
                if self.age < 18 {
                    println!("Do not serve alcohol to {}", self.name);
                }
            },
            VisitorAction::Probation => println!("{} is now a probationary member", self.name),
            VisitorAction::Refuse => println!("Do not allow {} in!", self.name)
        }
    }
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String },
    Refuse,
    Probation,
}

fn main() {
    let mut club_members = vec![
        Visitor::new("bert", VisitorAction::Accept, 45),
        Visitor::new("carl", VisitorAction::AcceptWithNote{ note: String::from("Soy milk is in the fridge.")}, 30),
        Visitor::new("hugo", VisitorAction::Refuse, 12),
    ];
    
    loop {
        println!("Hi, what's your name?");
    
        let name = get_user_name();
        
        let invited =
            club_members
                .iter()
                .find(|member| member.name == name);
    
        match invited {
            Some(member) => member.greet(),
            None => if name.is_empty() {
                break;
            } else {
                println!("{} is not on the member list.", name);
                club_members.push(Visitor::new(&name, VisitorAction::Probation, 0));
            }
        }
    }
    
    println!("The final member list:");
    println!("{:#?}", club_members);
    
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
