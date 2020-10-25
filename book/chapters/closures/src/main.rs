use std::thread;
use std::time::Duration;

use closures::Cacher;

fn main() {
    let simulated_user_specified_value = 20;
    let simulated_random_number = 4;

    generate_workout(simulated_user_specified_value, simulated_random_number);

    let x = vec![1, 2, 3];

    let equal_to_x = move |z| z == x;

    let y = vec![1, 2, 3];

    assert!(equal_to_x(y));
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        // thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_result.value(intensity));
        }
    }
}
