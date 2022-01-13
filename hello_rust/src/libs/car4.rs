use super::car2::Car;
use super::car3::car_factory;
use std::collections::HashMap;

pub fn run() {
    println!("\nCar4:");
    let mut orders: HashMap<i32, Car> = HashMap::new();
    // Declare a car as mutable "Car" struct
    let mut car: Car;
    let mut miles = 0;
    for order in  1..12 {
        car = car_factory(order, miles);
        orders.insert(order, car);
        println!("Car order {}: {:?}", order, orders.get(&order));

        // Reset miles for order variety
        if miles == 2100 {
            miles = 0;
        } else {
            miles = miles + 700;
        }
    }
}
