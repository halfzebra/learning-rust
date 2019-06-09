union IntOrFloat {
    i: i32,
    f: f32
}

fn main() {
    let mut iof = IntOrFloat{ i: 123 };
    unsafe
    {
        println!("unsafe union access {} ", iof.i);
    }

    unsafe
    {
        match iof
        {
            IntOrFloat { i } => println!("int {}", i),
            IntOrFloat { f } => println!("float {}", f),
        }
    }
}
