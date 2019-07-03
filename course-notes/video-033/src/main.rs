trait Printable
{
    fn format(&self) -> String;
}

impl Printable for i32
{
    fn format(&self) -> String
    {
        format!("i32: {}", *self)
    }
}

impl Printable for String
{
    fn format(&self) -> String
    {
        format!("String: {}", *self)
    }
}

fn print_value(v: &Printable)
{
    // the lookup of corresponding .format is happening at runtime.
    println!("{}", v.format())
}

fn main() {
    print_value(&"bye".to_string());
    print_value(&32);
}
