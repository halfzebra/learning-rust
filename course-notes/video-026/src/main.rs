#[derive(Debug)]
struct Point<T>
{
    x: T,
    y: T
}

#[derive(Debug)]
struct Line<T>
{
    a: Point<T>,
    b: Point<T>
}

fn main() {
    let p1 = Point{ x: 0, y: 0 };
    let p2 = Point{ x: 10, y: 30 };
    println!("{:?}", p1);
    let line = Line{ a: p1, b: p2 };
    println!("{:#?}", line);
}
