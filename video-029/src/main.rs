fn main() {
    let plus_one = |x:i32| -> i32 { x + 1 };
    let a = 6;

    println!("{} + 1 = {}", a, plus_one(a));

    let mut b = 2;
    {
        let plus_two = |x|
        {
            let mut z = x;
            z += b;
            z
        };
        println!("{} + 2 = {}", 3, plus_two(3));
    }

    // T       by value(copy)
    // &T      by reference
    // &mut T  my mutable reference

    // Borrow by mutable reference.
    println!("{}", &mut b);

    let mut c = 12;
    let plus_tree = |x: &mut i32| *x += 3;

    plus_tree(&mut c);
    println!("c = {}", c);
}
