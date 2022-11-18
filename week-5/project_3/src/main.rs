// program that displays the following menu for the food items available to take order from the customer.

use std::io;
use std::vec;

fn main() {
    loop {
        let mut input_order = String::new();
        let mut input_quantity = String::new();

        println!("                                                ");
        println!("                                                ");
        println!("MENU                 PRICE");
        println!("P = Poundo Yam/Edinkaiko Soup                 -#3,200");
        println!("F = Fried Rice & Chicken                      -#3,000");
        println!("A = Amala & Ewedu Soup                        -#2,500");
        println!("E = Eba & Egusi Soup                          -#2,000");
        println!("W = White Rice & Stew                         -#2,500");

        let order_select = vec!["P", "F", "A", "E", "W"];
        println!("                                                ");
        println!("                                                ");
        println!("To select your order from the menu use any of the letter in front of the food name");
        io::stdin().read_line(&mut input_order).expect("Not a valid string");
        let order = input_order.trim().to_uppercase();


        if order_select.iter().any(|e| order.contains(e)){
            println!("what is the quantity of food you want to buy");
            io::stdin().read_line(&mut input_quantity).expect("Not a valid string");
            let quantity:f64 = input_quantity.trim().parse().expect("Not a valid number");

            if order == "P" {
                let order_price = 3200.0 * quantity;
                if order_price > 10000.0 {
                   let percentage_discount = order_price * 0.05;
                   let discount = order_price - percentage_discount;
                   println!("The price of your Poundo Yam is #{} due to a 5% discount", discount);
                } else {
                    println!("The price of your Poundo Yam is #{}", order_price); 
                }
                break;
            } 

            else if order == "F" {
                let order_price = 3000.0 * quantity;
                if order_price > 10000.0 {
                   let percentage_discount = order_price * 0.05;
                   let discount = order_price - percentage_discount;
                   println!("The price of your Fried Rice & Chicken is #{} due to a 5% discount", discount);
                } else {
                    println!("The price of your Fried Rice & Chicken is #{}", order_price); 
                }
                break;
            } 

            else if order == "A" {
                let order_price = 2500.0 * quantity;
                if order_price > 10000.0 {
                   let percentage_discount = order_price * 0.05;
                   let discount = order_price - percentage_discount;
                   println!("The price of your Amala & Ewedu Soup is #{} due to a 5% discount", discount);
                } else {
                    println!("The price of your Amala & Ewedu Soup is #{}", order_price); 
                }
                break;
            } 

            else if order == "E" {
                let order_price = 2000.0 * quantity;
                if order_price > 10000.0 {
                   let percentage_discount = order_price * 0.05;
                   let discount = order_price - percentage_discount;
                   println!("The price of your Eba & Egusi Soup is #{} due to a 5% discount", discount);
                } else {
                    println!("The price of your Eba & Egusi Soup is #{}", order_price); 
                }
                break;
            } 

            else if order == "W" {
                let order_price = 2500.0 * quantity;
                if order_price > 10000.0 {
                   let percentage_discount = order_price * 0.05;
                   let discount = order_price - percentage_discount;
                   println!("The price of your White Rice & Stew is #{} due to a 5% discount", discount);
                } else {
                    println!("The price of your White Rice & Stew is #{}", order_price); 
                }
                break;
            } 
        } else {
            println!("Cannot take your order due to wrong order input");
            continue
        }
    }
}
