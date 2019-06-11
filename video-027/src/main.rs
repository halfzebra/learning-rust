fn print_u32(n: u32) {
    println!("The number is {}", n);
}

fn product(x: u32, y: u32) -> u32
{
    x * y
}

fn inc(n: &mut u32)
{
    *n += 1;
}

fn main() {
    let n1 = 10;
    let mut n2 = 15;
    print_u32(n1);
    print_u32(product(n1, 5));
    inc(&mut n2);
    print_u32(n2);
}
