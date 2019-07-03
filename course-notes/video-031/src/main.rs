#[derive(Debug)]
struct Point
{
    x: f64,
    y: f64
}

// It's possible to overload the addition fos adding different types of values:
// impl std::ops::Add<Vec> for Point
impl std::ops::Add for Point
{
    type Output = Point;

    fn add(self, other: Point) -> Point
    {
        Point{ x: self.x + other.x, y: self.y + self.y }
    }
}

fn main() {
    let a = Point{ x: 0.0, y: 34.2 };
    let b = Point{ x: 16.0, y: 1.0 };
    let sum = a + b;
    println!("{:?}", sum);
}
