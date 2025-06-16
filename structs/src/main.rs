/*
 * Structs in Rust are used to create custom data types that can hold multiple related values.
 * They are similar to classes in other languages but are more lightweight and do not support inheritance.
 * Structs can be mutable or immutable, and they can contain fields of different types.
 */

struct Car {
    owner: String,
    year: u32,
    fuel_level: f32,
    price: f32,
}

impl Car {
    // Associated functions (not tied to an instance)
    // These functions are called using the struct name, like Car::function_name()
    fn new(name: String, year: u32) -> Self {
        Car {
            owner: name,
            year: year,
            fuel_level: 0.0,
            price: 0.0,
        }
    }
    // Static method to calculate monthly insurance
    fn monthly_insurance() -> u32 {
        123
    }

    // Instance methods (tied to an instance of the struct)
    fn selling_price(&self) -> f32 {
        self.price + Car::monthly_insurance() as f32
    }

    fn display_car_info(&self) {
        println!("Car Owner: {}", self.owner);
        println!("Year: {}", self.year);
        println!("Fuel Level: {}%", self.fuel_level);
        println!("Price: ${:.2}", self.price);
    }

    fn fuel_car(&mut self, amount: f32) {
        if amount > 0.0 {
            self.fuel_level += amount;
            println!(
                "Fueled {} liters. New fuel level: {}%",
                amount, self.fuel_level
            );
        } else {
            println!("Invalid fuel amount: {}", amount);
        }
    }

    fn sell_car(&mut self, new_owner: String) {
        self.owner = new_owner;
        println!("Car sold to: {}", self.owner);
    }
}

fn main() {
    let mut my_car = Car {
        owner: String::from("Omar"),
        year: 2025,
        fuel_level: 75.5,
        price: 2_000_000.0,
    };

    let _car_year = my_car.year;
    // let _car_owner = my_car.owner; // String is heap allocated, ownership is moved here
    // If you want to keep the original string, you can clone it
    let _car_owner_clone = my_car.owner.clone(); // Cloning the string to keep the original intact
                                                 // Now you can use _car_owner_clone without affecting my_car.owner
    println!("Car owner: {}", _car_owner_clone);
    let _car_fuel_level = my_car.fuel_level;
    let _car_price = my_car.price;

    my_car.fuel_level = 50.0; // Update fuel level

    my_car.display_car_info(); // Display car information
    my_car.fuel_car(20.0); // Fuel the car
    my_car.display_car_info(); // Display updated car information
    my_car.sell_car(String::from("John")); // Sell the car to a new owner
    my_car.display_car_info(); // Display updated car information after sale

    let another_car = Car {
        owner: String::from("Alice"),
        year: my_car.year,
        fuel_level: my_car.fuel_level,
        price: my_car.price,
    };
    another_car.display_car_info(); // Display info of another car

    // Tuple struct
    struct Point_2D(f32, f32);
    struct Point_3D(f32, f32, f32);

    let point1 = Point_2D(3.0, 4.0);
    let point2 = Point_3D(1.0, 2.0, 3.0);
}
