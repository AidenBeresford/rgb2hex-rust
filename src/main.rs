use std::io;

struct Color(u8, u8, u8);

fn main() {
    
    let mut red_str = String::new();
    let mut blu_str = String::new();
    let mut grn_str = String::new();
    
    println!("RED-8: ");

    io::stdin()
        .read_line(&mut red_str)
        .expect("Failed to read stdin");

    let red = &red_str.trim().parse::<u8>().unwrap();

    println!("BLUE-8: ");

    io::stdin()
        .read_line(&mut blu_str)
        .expect("Failed to read stdin");

    let blu = &blu_str.trim().parse::<u8>().unwrap();

    println!("GREEN-8: ");

    io::stdin()
        .read_line(&mut gre_str)
        .expect("Failed to read stdin");

    let gre = &gre_str.trim().parse::<u8>().unwrap();

    user_color = Color(red, blu, gre);

}
