fn main() {
    println!("Hello, world!");
    /* Functions
    1. Generally are blocks of organised modular and reusable code
    2. Order in which the functions are defined doesn't matter 
    3. while naming the funcitons snake case is generally preferred
    */ 
    func_a();         // functions without arguments
    func_b(12,10.5);  // functions passing arguments 
}

fn func_a()
{
    println!("Func A");
}

fn func_b(x:i32, y:f64)
{
    println!("x = {}, y = {}",x, y);
}
