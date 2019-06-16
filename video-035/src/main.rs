fn main() {
    let v1 = vec![1,2,3];
    let v2 = v1;
    // borrow of moved value: `v1`
    // println!("{:?}", v1);

    let n1 = 1;
    let n2 = n1;
    println!("{}", n2);

    let a1 = Box::new(17);
    let a2 = a1;
    // borrow of moved value: `a1`
    // println!("{}", a1);

    // returning the value back, from a function:
    let print_vector = |v:Vec<i32>| -> Vec<i32>
    {
        println!("{:?}", v);
        v
    };
    let v3 = vec![1,2,3];
    let v4 = print_vector(v3);
}
