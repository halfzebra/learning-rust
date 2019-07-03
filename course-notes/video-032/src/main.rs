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

// compiler performs monomorphisation at compile time to decide which format to call.
fn print_value<T: Printable>(v: T)
{
    println!("{}", v.format())
}
// static dispatch preforms the lookup of corresponding implementation at compile time.
fn main() {
    println!("{}", 32.format());
    println!("{}", String::from("hello!").format());
    println!("{}", "hello world".to_string().format());

    print_value("bye".to_string());
}
