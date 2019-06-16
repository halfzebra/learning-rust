trait Shape
{
    fn area(&self) -> f64;
}

#[derive(Debug)]
struct Circle
{
    radius: f64
}

impl Shape for Circle 
{
    fn area(&self) -> f64
    {
        self.radius * self.radius * std::f64::consts::PI
    }
}

#[derive(Debug)]
struct Square
{
    side: f64
}

impl Shape for Square
{
    fn area(&self) -> f64
    {
        self.side * self.side
    }
}

fn main() {
    let l:[&Shape; 7] = [
        &Square{ side: 4.0 },
        &Circle{ radius: 1.0 },
        &Square{ side: 4.0 },
        &Circle{ radius: 0.5 },
        &Square{ side: 1.7 },
        &Circle{ radius: 200.3 },
        &Square{ side: 40.0 },
    ];

    for (i, x) in l.iter().enumerate() {
        println!("{}, {}", x.area(), i);
    }
}
