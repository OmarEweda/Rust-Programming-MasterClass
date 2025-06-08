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

    my_car.fuel_level = 80.0; // Update fuel level

    let another_car = Car {
        owner: String::from("Alice"),
        ..my_car
    };

    // Tuple struct
    struct Point_2D(f32, f32);
    struct Point_3D(f32, f32, f32);

    let point1 = Point_2D(3.0, 4.0);
    let point2 = Point_3D(1.0, 2.0, 3.0);
}
