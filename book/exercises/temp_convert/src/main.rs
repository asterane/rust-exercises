use std::io;

fn main() {
    // TODO Implement a program to convert temperatures between F and C
    println!("Temperature Conversion");

    loop {
        println!("Enter [f] for Fahrenheit, [c] for Celsius, or [q] to quit");

        let mut entry = String::new();

        io::stdin()
            .read_line(&mut entry)
            .expect("Failed to read selection");

        let set = entry.as_bytes()[0] as char;
        let set = set.to_lowercase().next();

        if set == Some('q') {
            break;
        } else if !(set == Some('c') || set == Some('f')) {
            continue;
        }

        println!("Input a temperature.");

        let mut temp = String::new();

        io::stdin()
            .read_line(&mut temp)
            .expect("Failed to read temperature");

        let temp: f32 = match temp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if set == Some('f') {
            let conv = (5.0 / 9.0) * (temp - 32.0);
            println!("{} F is equal to {:.1} C\n", temp, conv);
        } else {
            let conv = 32.0 + (9.0 / 5.0) * temp;
            println!("{} C is equal to {:.1} F\n", temp, conv);
        }
    }
}
