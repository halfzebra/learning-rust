fn main() {
    let x = 3.0;
    let y = 2.0;

    let result: Option<f64> =
        if y != 0.0 { Some(x/y) } else { None };

    // As expression.
    let with_default = match result
    {
        Some(v) => v, 
        _ => 0.0,
    };

    println!("{}", with_default);

    // {:?} marker prints the debug output.
    println!("Hello, world! {:?}", result);
}
