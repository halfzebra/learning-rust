fn main() {
    let mut v = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    println!("{:?}", v);

    v.push(44);

    println!("{:?}", v);

    println!("{:?}", v[0]);

    // It is impossible to use signed values as indices:
    // let idx:i32 = 0;
    // println!("{:?}", v[idx]);

    let idx:usize = 0;
    v[idx] = 0;
    println!("{:?}", v[idx]);

    // Thread panics when we read out of bounds:
    // println!("{:?}", v[100]);

    match v.get(6)
    {
        Some(v) => println!("The value is {}", v),
        None => println!("Got nothing!")
    }

    for x in &v
    {
        println!("{}", x);
    }

    v.push(77);

    let last = if let Some(i) = v.pop() { i } else { 10 };
    println!("Last element was {}", last);

    // Iterate over vector items using pattern matching:
    while let Some(x) = v.pop()
    {
        println!("{}", x);
    }
}
