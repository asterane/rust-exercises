use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Provide a space-separated list of integers.");
        return;
    }

    let mut list: Vec<i32> = args[1..]
        .iter()
        .map(|x| x.trim().parse().expect("Integers only"))
        .collect();

    list.sort();

    let len = list.len();
    let mean = list.iter().fold(0, |acc, x| acc + x) as f64 / (len as f64);

    let median = if len % 2 == 0 {
        (list[len / 2 - 1] + list[len / 2]) as f64 / 2.0
    } else {
        list[len / 2] as f64
    };

    let mut map = HashMap::new();
    for x in list {
        let count = map.entry(x).or_insert(0);
        *count += 1;
    }

    let max = *map.values().max().unwrap();
    let mode = map.iter().find(|(_, &val)| val == max).unwrap().0;

    println!("Mean: {:.3}\nMedian: {}\nMode: {}", mean, median, mode);
}
