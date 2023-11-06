use std::collections::HashMap;

// Define a Product struct to represent items in the inventory
#[derive(Debug, Clone)]
struct Product {
    name: String,
    description: String,
    price: f64,
    quantity: u32,
}

// Define an enum to represent transaction types
#[derive(Debug)]
enum TransactionType {
    Sale,
    Purchase,
}

// Define a Transaction struct to represent sales and purchases
#[derive(Debug)]
struct Transaction {
    product: Product,
    quantity: u32,
    price: f64,
    transaction_type: TransactionType,
}

// Define an Inventory struct to manage products
struct Inventory {
    products: HashMap<String, Product>,
}

impl Inventory {
    fn new() -> Self {
        Inventory {
            products: HashMap::new(),
        }
    }

    fn add_product(&mut self, product: Product) {
        self.products.insert(product.name.clone(), product);
    }

    fn sell_product(&mut self, product_name: &str, quantity: u32) -> Result<f64, String> {
        if let Some(product) = self.products.get_mut(product_name) {
            if product.quantity >= quantity {
                product.quantity -= quantity;
                let total_price = product.price * f64::from(quantity);
                Ok(total_price)
            } else {
                Err("Not enough stock".to_string())
            }
        } else {
            Err("Product not found".to_string())
        }
    }

    fn purchase_product(&mut self, product_name: &str, quantity: u32, cost: f64) {
        if let Some(product) = self.products.get_mut(product_name) {
            product.quantity += quantity;
        } else {
            let new_product = Product {
                name: product_name.to_string(),
                description: "".to_string(),
                price: 0.0,
                quantity,
            };
            self.add_product(new_product);
        }
    }

    fn generate_report(&self) {
        println!("Inventory Report:");
        for (name, product) in &self.products {
            println!(
                "Product: {}, Description: {}, Price: ${}, Quantity: {}",
                name, product.description, product.price, product.quantity
            );
        }
    }
}

fn main() {
    let mut inventory = Inventory::new();

    let product1 = Product {
        name: "Widget".to_string(),
        description: "A small widget".to_string(),
        price: 10.0,
        quantity: 100,
    };

    let product2 = Product {
        name: "Gadget".to_string(),
        description: "A cool gadget".to_string(),
        price: 20.0,
        quantity: 50,
    };

    inventory.add_product(product1);
    inventory.add_product(product2);

    // Simulate sales
    if let Ok(total_price) = inventory.sell_product("Widget", 5) {
        println!("Sale successful. Total price: ${}", total_price);
    } else {
        println!("Sale failed.");
    }

    // Simulate purchases
    inventory.purchase_product("Gadget", 10, 150.0);

    inventory.generate_report();
}
