It's possible to make `p1` available for `println!` later by deriving `Clone, Copy` traits.

```rust
#[derive(Debug, Clone, Copy)]
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
    let line = Line{ a: p1, b: p2 };
    println!("{:?}", p1);
    println!("{:#?}", line);
}
```