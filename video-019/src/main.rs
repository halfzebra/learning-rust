use std::mem;

fn main() {
    let mut a = [ 1, 2, 3 , 4, 5 ];
    println!("Array {:?} has length {}", a, a.len());
    a[0] = 255;
    println!("Array {:?}", a);

    if a != [ 1, 2, 3, 4, 5 ] {
        println!("Array have changed.")
    }

    // Initialize array of size 10 with value 1 for every element.
    let b = [1; 10];
    let c = [1u64; 10];

    println!("b = {:?}", b);

    for i in 0..b.len() {
        println!("{} {}", i, b[i]);
    }

    println!("size of b {} bytes", mem::size_of_val(&b));
    println!("size of c {} bytes", mem::size_of_val(&c));

    let m1 = [
        [ 1, 2, 3 ],
        [ 1, 2, 3 ],
    ];
    let m2:[[f32; 3]; 2] = [
        [ 1.0, 2.0, 3.0 ],
        [ 1.0, 2.0, 3.0 ],
    ];

    println!("m1 {:?}", m1);
    println!("m2 {:?}", m2);
}
