## Rusty Store Inventory Management System

### Overview

This is an inventory management system for a small retail store using Rust. The system allows to manage the store's inventory, sales, and purchases.

The system has the following features:

- *Inventory Management*: The system allows store managers to add by purchasing, edit, delete, and get information about products from the inventory. Each product has a name, description, quantity in stock, and prices of purchase and sale.

- *Sales Management*: The system allows store managers to record sales transactions, including the product sold, the quantity sold, and the sale price. The system also calculates and displays the total sales and profit made from each transaction.

- *Purchase Management*: The system allows store managers to record purchase transactions, including the product purchased, the quantity purchased, and the purchase price. The system also calculates and displays the total cost of each purchase.

- *Reporting*: The system allows to generate reports that show the store's inventory, sales, and purchase history. Reports are generated in a user-friendly text format that's easy to read, as a well-structured list.

- *Error Handling*: The system has robust error handling capabilities, including handling of invalid inputs, out-of-stock items, and other possible errors.

- *Security*: The system has basic security measures in place, such as authentication for store managers to prevent unauthorized access to the inventory, sales, and purchase data.

- *User Interface*: The system has a clear and intuitive text-based user interface that allows store managers to easily navigate and perform tasks.

### Getting Strarted

Download the code and navigate to the project directory.

To open the project documentation, run `cargo doc` and follow the link generated.

To start the program, run `cargo run`.
The program will prompt to enter a password:
```txt
Enter password, or x to escape:
```
After authentication, the user must enter the submenu number to proceed:
```txt
<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::
Enter feature number to go to, or x to escape:
Inventory Management  1
Sales Management      2
Purchase Management   3
Reporting             4
```

### Purchase Management

To add some products to the inventory system by purchasing them, the user must select `Purchase Management` by submitting option `3`. The program will prompt to enter new product details:

```txt
<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::
Enter product name to purchase, or x to escape:
```
```txt
<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::
Enter product description, or x to escape:
```
```txt
<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::
Enter product quantity, or x to escape:
```
```txt
<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::
Enter product sale price, or x to escape:
```
```txt
<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::
Enter product purchase price, or x to escape:
```

At the end, it will print a confirmation message with details of the purchase transaction and return to the main menu:
```txt
>>> Product added: PurchaseTx { product_name: "Potato", quantity: 100, purchase_price: 15.0 }; Total cost: 1500
```

After familiarizing themselves with the navigation and interaction with the program, users will be able to explore all its capabilities.

### Inventory Management

In this section, users can find several options for managing the products stored in the system:
```txt
<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::
Enter feature number to go to, or x to escape:
Get product       1
Edit product      2
Delete product    3
```
For example, getting information about a product looks like this:
```txt
<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::
Enter product name to get information about, or x to escape:
Potato
>>> Product information
>>> Name: Potato
>>> Description: Made in Ukraine
>>> Quantity in stock: 100
>>> Sale price: 15
>>> Purchase quantity and prices: [(100, 12.0)]
```
As for the product editing option, the program allows users to change only information about the product description and its sale price. But users are prohibited from manipulating quantity and purchase prices. This is done specifically for the consistency of the inventory management system.

It is easy to remove products, but it is better not to do it if users want to calculate the profit from sales of the given product when generating reports.

### Sales Management

In this section, users can simply sell products that are in stock as follows:
```txt
<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::
Enter product name to sell, or x to escape:
Potato
<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::
Product being sold: Product { name: "Potato", description: "Made in Ukraine", quantity: 100, sale_price: 15.0, purchase_prices: [(100, 12.0)] }
Enter product quantity, or x to escape:
2
>>> Product sold: SaleTx { product_name: "Potato", quantity: 2, sale_price: 15.0 }
```

### Reporting

This submenu contains several options for printing various reports that users can explore on their own:
```txt
<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::<>::
Enter feature number to go to, or x to escape:
Generate product report                   1
Generate sales report for each product    2
Display sales history                     3
Generate purchase report for each product 4
Display purchase history                  5
```
