mod pizza_order {
    pub struct Pizza {
        pub dough: String,
        pub cheese: String,
        pub toping: String,
    }

    impl Pizza {
        pub fn lunch(toping: &str) -> Pizza {
            Pizza {
                dough: String::from("Regular dough"),
                cheese: String::from("Mozzarella"),
                toping: String::from(toping),
            }
        }
    }
    pub mod help_customer {
        fn seat_at_table() {
            println!("Customer seated at table");
        }
        pub fn take_order() {
            seat_at_table();
            let cust_pizza: super::Pizza = super::Pizza::lunch("Vegigies");
            serve_customer(cust_pizza);
        }
        fn serve_customer(cust_pizza: super::Pizza) {
            println!(
                "The customer is served a regular pizza with {}",
                cust_pizza.toping
            );
        }
    }
}

pub fn order_food() {
    crate::restaurant::pizza_order::help_customer::take_order();
}
