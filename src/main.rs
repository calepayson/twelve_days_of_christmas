const DAILY_LINES: [&str; 12] = [
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
    "Twelve drummers drumming"
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
    println!("{OPENING_LINES}");
    println!("{}", DAILY_LINES[0]);
    println!("{}", DAYS[0])
}
