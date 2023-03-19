fn main() {
    for number in 1..13 {
        println!("On the {number} day of Christmas \n my true love sent to me:");
        if number == 1 {
            println!("A Partridge in a Pear Tree \n");
        }
        if number == 12 {
            println!("12 Drummers Drumming");
        }
        if number >= 11 {
            println!("11 Pipers Piping");
        }
        if number >= 10 {
            println!("10 Lords a Leaping");
        }
        if number >= 9 {
            println!("9 Ladies Dancing");
        }
        if number >= 8 {
            println!("8 Maids a Milking");
        }
        if number >= 7 {
            println!("7 Swans a Swimming");
        }
        if number >= 6 {
            println!("6 Geese a Laying");
        }
        if number >= 5 {
            println!("5 Golden Rings");
        }
        if number >= 4 {
            println!("4 Calling Birds");
        }
        if number >= 3 {
            println!("3 French Hens");
        }
        if number >= 2 {
            println!("2 Turtle Doves");
        }
        if number > 1 {
            println!("and a Partridge in a Pear Tree \n");
        }
    }
}
