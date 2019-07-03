fn main() {
    let msg:&'static str = "hello there"; 
    let msg:&str = "hello there"; // &str string slice
    println!("{}", msg);

    for c in msg.chars().rev() 
    {
        println!("{}", c);
    }

    if let Some(fst) = msg.chars().nth(0)
    {
        println!("The first character was {:?}", fst);
    }

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a <= ('z' as u8)
    {
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}", letters);

    // Deref conversion?
    let u:&str = &letters;

    // Concatenation
    let z = letters + "abc";
    println!("{}", z);

    let mut abc = "hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!");
    println!("{}", abc.replace("ello", "goodbye"));
}
