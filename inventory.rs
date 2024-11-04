//! This is a core module which implements the system
//! functionality.
use std::collections::HashMap;

/// Stores the entire Inventory Management System state in
/// RAM. It is reset every time the program is restarted.
/// Contains three vectors of products, sale and purchase
/// transactions.
pub struct Inventory {
    products: Vec<Product>,
    sale_txs: Vec<SaleTx>,
    purchase_txs: Vec<PurchaseTx>,
}

/// Structure for a product that includes information such
/// as:
/// - `name`: Product name (string)
/// - `description`: Product description (string)
/// - `quantity`: Quantity in stock (unsigned integer)
/// - `sale_price`: Sale price per unit (floating point 
/// number)
/// - `purchase_prices`: Vector of quantity and purchase
/// price per unit (tuple of unsigned integer and floating
/// point number)
#[derive(Debug, Clone)]
struct Product {
    name: String,
    description: String,
    quantity: u64,
    sale_price: f64,
    purchase_prices: Vec<(u64, f64)>,
}

/// Structure for recording sales information:
/// - `product_name`: Name of the sold product (string)
/// - `quantity`: Quantity of goods sold (unsigned integer)
/// - `sale_price`: Sale price per unit (floating point
/// number)
#[derive(Debug, Clone)]
struct SaleTx {
    product_name: String,
    quantity: u64,
    sale_price: f64,
}

/// Structure for recording purchase information:
/// - `product_name`: Name of the purchased product
/// (string)
/// - `quantity`: Quantity of purchased products (unsigned
/// integer)
/// - `purchase_price`: Purchase price per unit (floating
/// point number)
#[derive(Debug, Clone)]
struct PurchaseTx {
    product_name: String,
    quantity: u64,
    purchase_price: f64,
}

impl Inventory {
    /// Creates a new inventory struct to work with.
    pub fn new() -> Self {
        Inventory {
            products: Vec::new(),
            sale_txs: Vec::new(),
            purchase_txs: Vec::new(),
        }
    }
}

impl Product {
    fn new(name: String, description: String, quantity: u64, sale_price: f64, purchase_price: f64) -> Product {
        Product {
            name,
            description,
            quantity,
            sale_price,
            purchase_prices: vec![(quantity, purchase_price)],
        }
    }
}

impl SaleTx {
    fn new(product_name: String, quantity: u64, sale_price: f64) -> SaleTx {
        SaleTx {
            product_name,
            quantity,
            sale_price,
        }
    }
}

impl PurchaseTx {
    fn new(product_name: String, quantity: u64, purchase_price: f64) -> PurchaseTx {
        PurchaseTx {
            product_name,
            quantity,
            purchase_price,
        }
    }
}

trait InventoryManager {
    fn add_new_product(&mut self, name: String, description: String, quantity: u64, sale_price: f64, purchase_price: f64);
    fn add_same_product(&mut self, name: String, quantity: u64, purchase_price: f64) -> Result<(), String>;
    fn edit_product(&mut self, new_product: Product) -> Result<(), String>;
    fn delete_product(&mut self, product_name: &String);
    fn get_product(&self, product_name: &String) -> Option<&Product>;
    fn record_sale(&mut self, tx: SaleTx);
    fn record_purchase(&mut self, tx: PurchaseTx);
}

impl InventoryManager for Inventory {
    fn add_new_product(&mut self, name: String, description: String, quantity: u64, sale_price: f64, purchase_price: f64) {
        let new_product = Product::new(
            name,
            description,
            quantity,
            sale_price,
            purchase_price
        );
        self.products.push(new_product);
    }

    fn add_same_product(&mut self, name: String, quantity: u64, purchase_price: f64) -> Result<(), String> {
        match self.products.iter_mut().find(|p| p.name == name) {
            Some(product) => {
                product.quantity += quantity;
                for (q, p) in product.purchase_prices.iter_mut() {
                    if *p == purchase_price {
                        *q += quantity;
                        return Ok(());
                    }
                }
                product.purchase_prices.push((quantity, purchase_price));
                Ok(())
            }
            None => Err(format!("Unavailable product: {}", name))
        }
    }

    fn edit_product(&mut self, new_product: Product) -> Result<(), String> {
        match self.products.iter_mut().find(|p| p.name == new_product.name) {
            Some(product) => {
                *product = new_product;
                Ok(())
            }
            None => Err(format!("Unavailable product: {}", new_product.name))
        }
    }

    fn delete_product(&mut self, product_name: &String) {
        self.products.retain(|p| p.name != *product_name);
    }

    fn get_product(&self, product_name: &String) -> Option<&Product> {
        self.products.iter().find(|p| p.name == *product_name)
    }

    fn record_sale(&mut self, tx: SaleTx) {
        self.sale_txs.push(tx);
    }

    fn record_purchase(&mut self, tx: PurchaseTx) {
        self.purchase_txs.push(tx);
    }
}

/// Displays the Inventory Management submenu and prompts
/// the user for an option number to continue. To return to
/// the main menu, enter `x`.
pub fn inventory_handler(inventory: &mut Inventory) {
    let mut feature = String::new();
    loop {
        println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
        println!("Enter feature number to go to, or x to escape:");
        println!("Get product       1");
        println!("Edit product      2");
        println!("Delete product    3");
        feature.clear();
        std::io::stdin().read_line(&mut feature).unwrap();
        feature = feature.trim().to_string();
        if feature == "x" {
            return;
        }
        match &feature[..] {
            "1" => get_handler(inventory),
            "2" => edit_handler(inventory),
            "3" => delete_handler(inventory),
            _ => (),
        }
    }
}

/// Adds the product to the inventory according to the
/// parameters provided by the user.
fn add_handler(inventory: &mut Inventory) {
    let mut feature = String::new();

    // name
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Enter product name to purchase, or x to escape:");
    feature.clear();
    std::io::stdin().read_line(&mut feature).unwrap();
    feature = feature.trim().to_string();
    if feature == "x" {
        return;
    }

    if let Some(p) = inventory.get_product(&feature) {
        println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
        println!("Product already exists: {}", feature);
        println!("Enter any value to add more of this product, or x to escape:");
        feature.clear();
        std::io::stdin().read_line(&mut feature).unwrap();
        feature = feature.trim().to_string();
        if feature == "x" {
            return;
        }

        // same quantity
        println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
        println!("Enter product quantity, or x to escape:");
        feature.clear();
        std::io::stdin().read_line(&mut feature).unwrap();
        feature = feature.trim().to_string();
        if feature == "x" {
            return;
        }
        let quantity: u64;
        match feature.parse() {
            Ok(x) => {
                if x == 0 {
                    println!(">>> Invalid quantity: {}", x);
                    return;
                }
                quantity = x;
            },
            Err(e) => {
                println!(">>> Invalid quantity: {} ({})", feature, e);
                return;
            }
        }

        // same sale price
        println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
        println!("Enter product purchase price, or x to escape:");
        feature.clear();
        std::io::stdin().read_line(&mut feature).unwrap();
        feature = feature.trim().to_string();
        if feature == "x" {
            return;
        }
        let purchase_price: f64;
        match feature.parse() {
            Ok(x) => {
                if x < 0.0 {
                    println!(">>> Invalid sale price: {}", x);
                    return;
                }
                purchase_price = x;
            }
            Err(e) => {
                println!(">>> Invalid sale price: {} ({})", feature, e);
                return;
            }
        }

        let tx = PurchaseTx::new(p.name.clone(), quantity, purchase_price);
        match inventory.add_same_product(p.name.clone(), quantity, purchase_price) {
            Ok(_) => {
                println!(">>> Product added: {:?}; Total cost: {}", tx, tx.quantity as f64 * tx.purchase_price);
                inventory.record_purchase(tx);
            }
            Err(e) => println!(">>> {}", e)
        }
        return;
    }

    let name = feature.clone();

    // description
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Enter product description, or x to escape:");
    feature.clear();
    std::io::stdin().read_line(&mut feature).unwrap();
    feature = feature.trim().to_string();
    if feature == "x" {
        return;
    }
    let description = feature.clone();

    // quantity
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Enter product quantity, or x to escape:");
    feature.clear();
    std::io::stdin().read_line(&mut feature).unwrap();
    feature = feature.trim().to_string();
    if feature == "x" {
        return;
    }
    let quantity: u64;
    match feature.parse() {
        Ok(x) => {
            if x == 0 {
               println!(">>> Invalid quantity: {}", x);
               return; 
            }
            quantity = x;
        }
        Err(e) => {
            println!(">>> Invalid quantity: {} ({})", feature, e);
            return;
        }
    }

    // sale price
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Enter product sale price, or x to escape:");
    feature.clear();
    std::io::stdin().read_line(&mut feature).unwrap();
    feature = feature.trim().to_string();
    if feature == "x" {
        return;
    }
    let sale_price: f64;
    match feature.parse() {
        Ok(x) => {
            if x < 0.0 {
                println!(">>> Invalid sale price: {}", x);
                return;
            }
            sale_price = x;
        }
        Err(e) => {
            println!(">>> Invalid sale price: {} ({})", feature, e);
            return;
        }
    }

    // purchase price
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Enter product purchase price, or x to escape:");
    feature.clear();
    std::io::stdin().read_line(&mut feature).unwrap();
    feature = feature.trim().to_string();
    if feature == "x" {
        return;
    }
    let purchase_price: f64;
    match feature.parse() {
        Ok(x) => {
            if x < 0.0 {
                println!(">>> Invalid purchase price: {}", x);
                return;
            }
            purchase_price = x;
        }
        Err(e) => {
            println!(">>> Invalid purchase price: {} ({})", feature, e);
            return;
        }
    }

    let tx = PurchaseTx::new(name.clone(), quantity, purchase_price);
    inventory.add_new_product(name.clone(), description, quantity, sale_price, purchase_price);
    println!(">>> Product added: {:?}; Total cost: {}", tx, tx.quantity as f64 * tx.purchase_price);
    inventory.record_purchase(tx);
}

/// Edits product information, particularly the description
/// or sale price.
fn edit_handler(inventory: &mut Inventory) {
    let mut feature = String::new();
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Enter product name to edit, or x to escape:");
    feature.clear();
    std::io::stdin().read_line(&mut feature).unwrap();
    feature = feature.trim().to_string();
    if feature == "x" {
        return;
    }
    let product = match inventory.get_product(&feature) {
        Some(p) => p,
        None => {
            println!(">>> Unavailable product: {}", feature);
            return;
        }
    };

    let mut new_product = product.clone();
    
    // description
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Product being set: {:?}", new_product);
    println!("Enter product description, or c to continue, or press x to escape:");
    feature.clear();
    std::io::stdin().read_line(&mut feature).unwrap();
    feature = feature.trim().to_string();
    if feature == "x" {
        return;
    }
    if feature != "c" {
        new_product.description = feature.clone();
    }
        
    // sale price
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Product being set: {:?}", new_product);
    println!("Enter product sale price, or c to continue, or x to escape:");
    feature.clear();
    std::io::stdin().read_line(&mut feature).unwrap();
    feature = feature.trim().to_string();
    if feature == "x" {
        return;
    }
    if feature != "c" {
        match feature.parse() {
            Ok(x) => {
                if x < 0.0 {
                    println!(">>> Invalid sale price: {}", x);
                    return;
                }
                new_product.sale_price = x;
            },
            Err(e) => {
                println!(">>> Invalid sale price: {} ({})", feature, e);
                return;
            }
        }
    }

    match inventory.edit_product(new_product.clone()) {
        Ok(_) => println!(">>> Product edited: {:?}", new_product),
        Err(e) => println!(">>> {}", e)
    }
}

/// Removes a product from the system.
fn delete_handler(inventory: &mut Inventory) {
    let mut feature = String::new();
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Enter product name to delete, or x to escape:");
    feature.clear();
    std::io::stdin().read_line(&mut feature).unwrap();
    feature = feature.trim().to_string();
    if feature == "x" {
        return;
    }
    inventory.delete_product(&feature);
    println!(">>> Product deleted if existed: {}", feature);
}

/// Displays information about the product.
fn get_handler(inventory: &Inventory) {
    let mut feature = String::new();
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Enter product name to get information about, or x to escape:");
    feature.clear();
    std::io::stdin().read_line(&mut feature).unwrap();
    feature = feature.trim().to_string();
    if feature == "x" {
        return;
    }
    match inventory.get_product(&feature) {
        Some(product) => {
            println!(">>> Product information");
            println!(">>> Name: {}", product.name);
            println!(">>> Description: {}", product.description);
            println!(">>> Quantity in stock: {}", product.quantity);
            println!(">>> Sale price: {}", product.sale_price);
            println!(">>> Purchase quantity and prices: {:?}\n", product.purchase_prices);
        },
        None => println!(">>> Unavailable product: {}", feature)
    }
}

/// Allows the user to sell products available in the system.
pub fn sales_handler(inventory: &mut Inventory) {
    let mut feature = String::new();
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Enter product name to sell, or x to escape:");
    feature.clear();
    std::io::stdin().read_line(&mut feature).unwrap();
    feature = feature.trim().to_string();
    if feature == "x" {
        return;
    }
    let product = match inventory.get_product(&feature) {
        Some(p) => p,
        None => {
            println!(">>> Unavailavle product: {}", feature);
            return;
        }
    };

    let mut new_product = product.clone();
    
    // quantity
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Product being sold: {:?}", new_product);
    println!("Enter product quantity, or x to escape:");
    feature.clear();
    std::io::stdin().read_line(&mut feature).unwrap();
    feature = feature.trim().to_string();
    if feature == "x" {
        return;
    }
    let quantity: u64;
    match feature.parse() {
        Ok(x) => quantity = x,
        Err(e) => { println!(">>> Invalid quantity: {} ({})", feature, e); return; }
    }
    if quantity > new_product.quantity {
        println!(">>> Invalid quantity: {}", quantity);
        return;
    }

    new_product.quantity -= quantity;
    let tx = SaleTx::new(new_product.name.clone(), quantity, new_product.sale_price);
    match inventory.edit_product(new_product) {
        Ok(_) => {
            println!(">>> Product sold: {:?}", tx);
            inventory.record_sale(tx);
        }
        Err(e) => println!("{}", e)
    }
}

/// Allows the user to purchase products and store them in the system.
pub fn purchase_handler(inventory: &mut Inventory) {
    add_handler(inventory);
}

/// Allows the user to generate reports. Displays the
/// Reporting submenu and prompts the user for an option
/// number to continue. To return to the main menu, enter
/// `x`.
pub fn report_handler(inventory: &mut Inventory) {
    let mut feature = String::new();
    loop {
        println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
        println!("Enter feature number to go to, or x to escape:");
        println!("Generate product report                   1");
        println!("Generate sales report for each product    2");
        println!("Display sales history                     3");
        println!("Generate purchase report for each product 4");
        println!("Display purchase history                  5");
        feature.clear();
        std::io::stdin().read_line(&mut feature).unwrap();
        feature = feature.trim().to_string();
        if feature == "x" {
            return;
        }
        match &feature[..] {
            "1" => report_products(inventory),
            "2" => report_sales(inventory),
            "3" => display_sales(inventory),
            "4" => report_purchases(inventory),
            "5" => display_purchases(inventory),
            _ => (),
        }
    }
}

/// Displays a report of products.
fn report_products(inventory: &mut Inventory) {
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Product report");
    for product in inventory.products.iter() {
        println!("Product: {}", product.name);
        println!("Description: {}", product.description);
        println!("Quantity in stock: {}", product.quantity);
        println!("Sale price: {}", product.sale_price);
        println!("Purchase quantity and prices: {:?}", product.purchase_prices);
        println!("= = = = = = = = = = = = = = = = = = = = = = = = = = = = = =");
    }
}

/// Displays a report of sales grouped by product, and total revenue.
fn report_sales(inventory: &mut Inventory) {
    let mut total_sales: HashMap<String, (u64, f64)> = HashMap::new();
    for tx in inventory.sale_txs.iter() {
        let sale = total_sales.entry(tx.product_name.clone()).or_insert((0, 0.0));
        sale.0 += tx.quantity;
        sale.1 += tx.quantity as f64 * tx.sale_price;
    }
    let mut revenue: f64 = 0.0;
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Sales report");
    for (k, v) in total_sales.iter() {
        print!("Product: {}; Quantity: {}; Total sale price: {}; Profit: ", k, v.0, v.1);
        if let Some(product) = inventory.products.iter().find(|p| p.name == *k) {
            let mut sold_quantity = v.0;
            let mut purchase_price: f64 = 0.0;
            for (q, p) in product.purchase_prices.iter() {
                let current_quantity = sold_quantity.min(*q);
                purchase_price += current_quantity as f64 * p;
                sold_quantity -= current_quantity;
                if sold_quantity == 0 {
                    break;
                }
            }
            println!("{}", v.1 - purchase_price);
            revenue += v.1 - purchase_price;
        } else {
            println!("Error (Unable to calculate)");
        }
    }
    println!("Total Revenue: {}", revenue);
}

/// Displays a history of sales and profit from each transaction.
fn display_sales(inventory: &mut Inventory) {
    struct PurchasePrices {
        offset: u64,
        purchase_prices: Vec<(u64, f64)>,
    }
    let mut sold_purchase_prices: HashMap<String, PurchasePrices> = HashMap::new();
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Sales history");
    for tx in inventory.sale_txs.iter() {
        let mut err: Option<String> = None;
        if !sold_purchase_prices.contains_key(&tx.product_name) {
            if let Some(product) = inventory.products.iter().find(|p| p.name == tx.product_name) {
                sold_purchase_prices.insert(
                    product.name.clone(),
                    PurchasePrices {
                        offset: 0,
                        purchase_prices: product.purchase_prices.clone()
                    }
                );
            } else {
                err = Some("Error (Unable to calculate)".to_string());
            }
        }
        let mut purchase_price: f64 = 0.0;
        if sold_purchase_prices.contains_key(&tx.product_name) {
            let product = sold_purchase_prices.get_mut(&tx.product_name).unwrap();
            let mut tx_quantity = tx.quantity;
            let mut skip: u64 = product.offset;
            for (q, p) in product.purchase_prices.iter() {
                if skip >= *q {
                    skip -= *q;
                    continue;
                }
                let current_quantity = tx_quantity.min(*q - skip);
                skip = 0;
                purchase_price += current_quantity as f64 * p;
                tx_quantity -= current_quantity;
                if tx_quantity == 0 {
                    break;
                }
            }
            product.offset += tx.quantity;
        }
        match err {
            Some(e) => {
                println!(
                    "Product: {}; Quantity: {}; Sale price: {}; Profit: {}",
                    tx.product_name,
                    tx.quantity,
                    tx.sale_price,
                    e);
            },
            None => {
                println!(
                    "Product: {}; Quantity: {}; Sale price: {}; Profit: {}",
                    tx.product_name,
                    tx.quantity,
                    tx.sale_price,
                    tx.quantity as f64 * tx.sale_price - purchase_price);
            }
        }
    }
}

/// Displays a report of purchases grouped by product.
fn report_purchases(inventory: &mut Inventory) {
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Purchases report");
    for product in inventory.products.iter() {
        let mut quantity: u64 = 0;
        let mut purchase_price: f64 = 0.0;
        for (q, p) in product.purchase_prices.iter() {
            quantity += *q;
            purchase_price += *q as f64 * p;
        }
        println!("Product: {}; Quantity: {}; Total purchase price: {}", product.name, quantity, purchase_price);
    }
}

/// Displays a history of purchases.
fn display_purchases(inventory: &mut Inventory) {
    println!("<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::");
    println!("Purchase history");
    for tx in inventory.purchase_txs.iter() {
        println!(
            "Product: {}; Quantity: {}; Purchase price: {}; Total cost: {}",
            tx.product_name,
            tx.quantity,
            tx.purchase_price,
            tx.quantity as f64 * tx.purchase_price);
    }
}
