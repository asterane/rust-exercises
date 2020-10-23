use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Provide index of desired Fibonacci number.");
        return
    }

    let ind: u32 = match args[1].trim().parse() {
        Ok(val) => val,
        Err(_) => { println!("Provide index of desired Fibonacci number.");
                    return },
    };

    let (mut acc, mut last) = (0, 1);
    let mut next;

    for _ in 0..ind {
        next = acc + last;
        last = acc;
        acc = next;
    }

    println!("{}", acc);
}
