fn main() {
    let gifts = ["partridge in a pear tree", "turtle doves and", "french hens",
                 "calling birds", "golden rings", "geese a-laying",
                 "swans a-swimming", "maids a-milking", "ladies dancing",
                 "lords a-leaping", "pipers piping", "drummers drumming"];
    
    let mut suffix: &str;

    for i in 1..13 {
        
        suffix = match i {
            1 => "st",
            2 => "nd",
            3 => "rd",
            _ => "th"
        };

        println!("On the {}{} day of Christmas, my true love sent to me", i, suffix);

        for j in (0..i).rev() {
            if j == 0 {
                println!("A {}", gifts[j])
            } else{
                println!("{} {}", j+1, gifts[j])
            }
            
        }

        println!("\r")
    }
}
