use std::io;
use colored::Colorize;

struct Color8(u8, u8, u8);

fn main() {
    let mut red_str = String::new();
    let mut blu_str = String::new();
    let mut grn_str = String::new();
    
    println!("RED-8: ");

    io::stdin()
        .read_line(&mut red_str)
        .expect("Failed to read stdin");

    let red = &red_str.trim().parse::<u8>().unwrap();

    println!("GREEN-8: ");     
     
    io::stdin()
        .read_line(&mut grn_str)
        .expect("Failed to read stdin");
 
    let grn = &grn_str.trim().parse::<u8>().unwrap();

    println!("BLUE-8: ");

    io::stdin()
        .read_line(&mut blu_str)
        .expect("Failed to read stdin");
    let blu = &blu_str.trim().parse::<u8>().unwrap();

    let text_color = Color8(*red, *grn, *blu);

    println!("Hexadecimal Color Code: {}", rgb_2_hex(text_color.0, text_color.1, text_color.2)
             .truecolor(text_color.0, text_color.1, text_color.2));

}

fn rgb_2_hex(r: u8, g: u8, b: u8) -> String  {
    let mut hex_str = String::from("");

    hex_str.push_str(&hex::encode(vec![r, g, b]));

    return hex_str;
}
