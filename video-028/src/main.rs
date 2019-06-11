#[derive(Debug)]
struct Point
{
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct Line
{
    a: Point,
    b: Point
}

impl Line
{
    fn len(&self) -> f64
    {
        let dx = self.a.x - self.b.x;
        let dy = self.a.y - self.b.y;
        (dx * dx + dy * dy).sqrt()
    }

    fn move_y(&mut self, offset: f64) {
        self.a.y += offset;
        self.b.y += offset;
    }
}

fn main() {
    let mut line = Line{
        a: Point{ x: 0.0, y: 0.0 },
        b: Point{ x: 15.0, y: 3.0 }
    };
    println!("{:#?}", line);
    println!("The length is {}", line.len());
    line.move_y(3.0);
    println!("{:#?}", line);
}
