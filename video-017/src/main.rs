enum Color
{
    Red,
    Green,
    Blue,
    Rgb(u8, u8, u8),
    Cmyk{cyan: u8, magenta:u8, yellow: u8, black: u8}
}

fn main() {
    // let color = Color::Red;
    // let color = Color::RgbColor(100, 255, 255);
    let color = Color::Cmyk{cyan: 0, magenta:0, yellow: 0, black: 255};

    match color 
    {
        Color::Red => println!("It was red all along"),
        Color::Green => println!("It was green all along"),
        Color::Blue => println!("It was blue all along"),
        Color::Rgb(100, y, z) => println!("RGB 100 {} {}", y, z),
        Color::Rgb(x, y, z) => println!("RGB {} {} {}", x, y, z),
        Color::Cmyk{cyan: _, magenta: _, yellow: _, black: 255} => println!("is black"),
        _ => println!("whatever")
    }
}
