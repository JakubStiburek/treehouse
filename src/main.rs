use std::io::stdin;
use std::num::ParseIntError;

//
// #[derive(Debug)]
// struct Customer {
//     cart: Vec<Stock_item>,
//     wallet: u32,
// }
// impl Customer {
//     fn new(wallet: u32) -> Self {
//         Self {
//             cart,
//             wallet
//         }
//     }
//
//     fn do_action(&mut self, action: CustomerAction, &item: Option<StockItem>) {
//         match action {
//             CustomerAction::ListCart {} => println!("{:#?}", self.cart),
//             CustomerAction::Add {item} => {
//                 match item {
//                     Some(item) => self.cart.push(item),
//                     None => println!("Cannot add.")
//                 }
//             },
//             CustomerAction::Remove {item} => {
//                 match item {
//                     Some(item) => {
//                         let index = self.cart
//                             .iter()
//                             .position(|i| i == item );
//                         match index {
//                             Some(index) => self.cart.remove(index),
//                             None => println!("This item is not in the cart")
//                         }
//                     }
//                     None => println!("Cannot remove.")
//                 }
//             },
//             CustomerAction::Pay {} => {
//                 let total_cost = self.cart
//                     .iter()
//                     .map(|i| i.cost)
//                     .reduce(accum, cost);
//
//                 match total_cost {
//                     Some(cost) => {
//                         if cost < wallet {
//                             println!("You have bought:");
//                             println!("{:#?}", self.cart);
//                             self.cart.clear();
//                         }
//                     }
//                     None => println!("Oops, the cart is empty")
//                 }
//             }
//         }
//     }
// }
//
// #[derive(Debug)]
// enum CustomerAction {
//     ListCart,
//     Add { item: Option<StockItem> },
//     Remove { item: Option<Stock_item> },
//     Pay,
// }
//
// #[derive(Debug)]
// struct StockItem {
//     name: String,
//     cost: u32,
// }
// impl StockItem {
//     fn new(name: &str, cost: u32) -> Self {
//         Self {
//             name: name.to_lowercase(),
//             cost
//         }
//     }
// }

fn main() {
    println!("Hi. Do you have money?");
    let customer_wallet = get_customer_wallet();
    
    match customer_wallet {
        Ok(balance) => {
            if balance > 0 {
                println!("I see you have full sack. Is that {} coins?", balance)
            } else {
                println!("Oh, get lost beggar.")
            }
        },
        Err(_) => println!("Oh, get lost beggar.")
    }
}

fn get_customer_wallet() -> Result<u32, ParseIntError> {
    let mut wallet = String::new();
    stdin()
        .read_line(&mut wallet)
        .expect("Failed to read line");
    let balance = wallet
        .trim()
        .to_lowercase()
        .parse::<u32>();
    balance
}
