const GIFTS: [&str; 12] = [
    "And a partridge in a pear tree",
    "Two turle doves,",
    "Three French hens,",
    "Four calling birds,",
    "Five golden rings,",
    "Six geese a-laying,",
    "Seven swans a-swimming,",
    "Eight maids a-milking,",
    "Nine ladies dancing,",
    "Ten lords a-leeping,",
    "Eleven pipers piping,",
    "Twelve drummers drumming,"
];

const DAYS: [&str; 12] = [
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
    "tenth",
    "eleventh",
    "twelfth"
];

fn main() {
    for i in 0..12 {
        print_opening_line(i);

        if i == 0 {
            println!("A partridge in a pair tree")
        } else {
            print_gifts(i);
        }

        println!("");
    }
}

fn print_opening_line(index: usize) {
    println!("On the {} day of Christmas,\nmy true love gave to me", DAYS[index]);
}

fn print_gifts(index: usize) {
    for i in (0..(index + 1)).rev() {
        println!("{}", GIFTS[i])
    }
}