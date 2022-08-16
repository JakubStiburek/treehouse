use std::io::stdin;
use std::num::ParseIntError;


#[derive(Debug)]
struct Customer {
    cart: Vec<StockItem>,
    wallet: u32,
}
impl Customer {
    fn new(wallet: u32) -> Self {
        Self {
            cart: vec![],
            wallet
        }
    }

    fn do_action(&mut self, action: CustomerAction) {
        match action {
            CustomerAction::ListCart {} => println!("{:#?}", self.cart),
            CustomerAction::Add {item} => {
                match item {
                    Some(item) => {
                        let new_item = StockItem::new( item.name.as_str(), item.cost);
                        self.cart.push(new_item) },
                    None => { println!("Cannot add.") }
                }
            },
            CustomerAction::Remove {item} => {
                match item {
                    Some(item) => {
                        let index = self.cart
                            .iter()
                            .position(|i| i.name == item.name );
                        match index {
                            Some(index) => { self.cart.remove(index); }
                            None => { println!("This item is not in the cart") }
                        }
                    }
                    None => { println!("Cannot remove.") }
                }
            },
            CustomerAction::Pay {} => {
                let total_cost = self.cart
                    .iter()
                    .map(|i| i.cost)
                    .reduce(|accum, item| {
                        accum
                    });

                match total_cost {
                    Some(cost) => {
                        if cost < self.wallet {
                            println!("You have bought:");
                            println!("{:#?}", self.cart);
                            self.cart.clear();
                        }
                    }
                    None => println!("Oops, the cart is empty")
                }
            }
        }
    }
}

#[derive(Debug)]
enum CustomerAction<'a> {
    ListCart,
    Add { item: Option<&'a StockItem> },
    Remove { item: Option<&'a StockItem> },
    Pay,
}

#[derive(Debug)]
struct StockItem {
    name: String,
    cost: u32,
}
impl StockItem {
    fn new(name: &str, cost: u32) -> Self {
        Self {
            name: name.to_lowercase(),
            cost
        }
    }
}

fn main() {
    let stock = vec![
        StockItem::new("water", 10),
        StockItem::new("oil", 5),
        StockItem::new("meat", 20),
        StockItem::new("vegetables", 8),
        StockItem::new("sword", 100),
        StockItem::new("helmet", 250),
    ];
    
    println!("Hi. Do you have money?");
    let customer_wallet = get_customer_wallet();
    
    match customer_wallet {
        Ok(balance) => {
            if balance > 0 {
                println!("I see you have full sack. Is that {} coins?", balance);
                println!("Have a look at my stock:");
                list_stock(&stock);
                let mut customer = Customer::new(balance);
                customer.do_action(CustomerAction::Add { item: Some(&stock[0]) });
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

fn list_stock(stock: &Vec<StockItem>) {
    println!("{:#?}", stock)
}
