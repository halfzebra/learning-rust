trait LiveBeing
{
    fn create(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    
    fn talk(&self)
    {
        println!("{} can not talk", self.name())
    }
}

#[derive(Debug)]
struct Cat
{
    name: &'static str,
}

impl LiveBeing for Cat {
    fn create(name: &'static str) -> Cat
    {
        Cat{name: name}
    }

    fn name(&self) -> &'static str {
        self.name
    }
}

#[derive(Debug)]
struct Human
{
    name: &'static str,
}

impl LiveBeing for Human {
    fn create(name: &'static str) -> Human
    {
        Human{name: name}
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn talk(&self) {
        println!("{} says hello!", self.name)
    }
}

// Extend built-in with a trait.
trait Summary<T>
{
    fn sum(&self) -> T;
}

impl Summary<i32> for Vec<i32>
{
    fn sum(&self) -> i32
    {
        let mut res = 0;
        for x in self
        {
            res += x
        }
        res
    }
}

fn main() {
    let person = Human{ name: "Jose" };
    let cat = Cat{ name: "Boris" };
    let another_cat = Cat::create("John");
    let another_human: Human = LiveBeing::create("John");
    println!("{:#?}", person);
    println!("{:#?}", cat);
    println!("{:#?}", another_cat);
    println!("{:#?}", another_human);

    person.talk();
    cat.talk();

    let v = vec![1,2,3];
    println!("sum of elements in vector {:?} is {}", v, v.sum());
}
