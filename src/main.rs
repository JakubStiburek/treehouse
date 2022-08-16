use std::io::stdin;
use std::num::ParseIntError;


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
            CustomerAction::ListCart {} => {
                println!("Your cart:");
                list_items(&self.cart);
            },
            CustomerAction::Add {item} => {
                match item {
                    Some(item) => {
                        let new_item = StockItem::new( item.name.as_str(), item.cost);
                        self.cart.push(new_item);
                        println!("Your cart:");
                        list_items(&self.cart)
                    },
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
                            Some(index) => {
                                self.cart.remove(index);
                                println!("Your cart:");
                                list_items(&self.cart);
                            }
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
                    .reduce(|accum, _item| {
                        accum
                    });

                match total_cost {
                    Some(cost) => {
                        if cost <= self.wallet {
                            println!("You have bought:");
                            list_items(&self.cart);
                            self.cart.clear();
                            self.wallet = self.wallet - cost;
                        } else {
                            println!("Not enough money. Don't worry... My cousin offers great loans!")
                        }
                    }
                    None => println!("Oops, the cart is empty")
                }
            }
        }
    }
}

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
                    loop {
                        println!("Have a look at my stock:");
                        list_items(&stock);
                        let mut customer = Customer::new(balance);
                        customer.do_action(CustomerAction::ListCart);
                        add_items_loop(&mut customer, &stock);
                        remove_items_loop(&mut customer, &stock);
                        println!("Ok, let's see what you got there.");
                        customer.do_action(CustomerAction::Pay);
                        if customer.wallet == 0 {
                            break;
                        }
                    }
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

fn select_item() -> String {
    let mut selected = String::new();
    stdin()
        .read_line(&mut selected)
        .expect("Failed to read line");
    selected
        .trim()
        .to_lowercase()
}

fn find_stock_item(stock: &Vec<StockItem>, item_name: String) -> Option<&StockItem> {
    let found =
        stock
            .iter()
            .find(|item| item.name == item_name);
    found
}

fn add_items_loop(customer: &mut Customer, stock: &Vec<StockItem>) {
    loop {
        println!("What would you like to buy?");
        let selected = select_item();
        if selected.len() <= 0 {
            customer.do_action(CustomerAction::ListCart);
            break;
        }
        let from_stock = find_stock_item(&stock, selected);
        customer.do_action(CustomerAction::Add { item: from_stock});
    }
}

fn remove_items_loop(customer: &mut Customer, stock: &Vec<StockItem>) {
    loop {
        println!("Have you changed your mind?");
        let selected = select_item();
        if selected.len() <= 0 {
            customer.do_action(CustomerAction::ListCart);
            break;
        }
        let from_stock = find_stock_item(&stock, selected);
        customer.do_action(CustomerAction::Remove { item: from_stock});
    }
}

fn list_items(stock: &Vec<StockItem>) {
    let mut index = 0;
    for item in stock {
        println!("No. {}", index);
        println!("{}", item.name);
        println!("{}", item.cost);
        println!("---");
        index = index + 1;
    }
}
