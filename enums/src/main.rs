/**
 * Enums in Rust are a powerful way to define a type that can be one of several different variants.
 * This example demonstrates how to use enums to represent different days of the week and various travel modes.     
 * The `Weekday` enum represents the days of the week, while the `TravelMode` enum represents different modes of transportation,
 * each with an associated floating-point value (e.g., distance or speed).
 * The `travel_allowance` method calculates the travel allowance based on the mode of transport and distance.
 * The `main` function demonstrates how to create instances of these enums and call the method to calculate the allowance.
 */

enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

enum TravelMode {
    Car(f32),
    Bike(f32),
    Bus(f32),
    /// Represents a train with a specific floating-point value, such as speed or distance.
    Train(f32),
}

/// Calculates the travel allowance based on the travel mode and distance.
///
/// # Returns
///
/// * `f32` - The calculated allowance amount, which depends on the travel mode:
///     - `Car`: allowance is `miles * 0.5`
///     - `Bike`: allowance is `miles * 3.0`
///     - `Bus`: allowance is `miles * 0.3`
///     - `Train`: allowance is `miles * 0.4`
///
/// # Examples
///
/// ```
/// let mode = TravelMode::Car(100.0);
/// assert_eq!(mode.travel_allowance(), 50.0);
/// ```
impl TravelMode {
    fn travel_allowance(&self) -> f32 {
        let allowance = match self {
            TravelMode::Car(miles) => miles * 0.5,
            TravelMode::Bike(miles) => miles * 3.0,
            TravelMode::Bus(miles) => miles * 0.3,
            TravelMode::Train(miles) => miles * 0.4,
        };
        allowance
    }
}
fn main() {
    let today = Weekday::Sunday;
    let mode_of_transport = TravelMode::Car(100.0);
    println!(
        "Allowance for traveling 100 miles is: ${}",
        mode_of_transport.travel_allowance()
    );
}
