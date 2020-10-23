use std::io;

fn main() {
    println!("Enter a string for conversion.");

    let mut string = String::new();

    io::stdin().read_line(&mut string).expect("Failure");

    let mut out = String::new();

    for w in string.split_whitespace() {
        let mut next = w.to_string().to_lowercase();

        if next.starts_with(|x| {
            if x == 'a' || x == 'e' || x == 'i' || x == 'o' || x == 'u' {
                true
            } else { false }
        }) {
            next.push_str("-hay");
        } else {
            let start = next.remove(0);
            next.push('-');
            next.push(start);
            next.push_str("ay");
        }

        out.push_str(&next);
        out.push(' ');
    }

    println!("{}", out);
}
