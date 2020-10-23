fn main() {
    let nums: [&str; 12] = ["first", "second", "third", "fourth",
                           "fifth", "sixth", "seventh", "eighth",
                           "ninth", "tenth", "eleventh", "twelfth"];

    let lines: [&str; 12] = ["And a partridge in a pear tree",
                            "Two turtle doves",
                            "Three French hens",
                            "Four calling birds",
                            "Five gold rings",
                            "Six geese a laying",
                            "Seven swans a swimming",
                            "Eight maids a milking",
                            "Nine ladies dancing",
                            "Ten lords a leaping",
                            "Eleven pipers piping",
                            "Twelve drummers drumming"];

    println!("Twelve Days of Christmas\n");

    for i in 0..12 {
        println!("On the {} day of Christmas", nums[i]);
        println!("My true love gave to me");

        if i == 0 {
            println!("A partridge in a pear tree")
        } else {
            for j in 0..(i + 1) {
                println!("{}", lines[ i - j ])
            }
        }

        println!("")
    }
}
