fn main() {
    println!("Hello, world!");
    /* Functions
    1. Generally are blocks of organised modular and reusable code
    2. Order in which the functions are defined doesn't matter 
    3. while naming the funcitons snake case is generally preferred
    */ 
    func_a();         // functions without arguments
    func_b(12,10.5);  // functions passing arguments 

    /* Statements and Expressions
    Statements are instructions tat perform some action and do not return a value.
    Expressions evaluate tp a resulting value. */
    /* let y = 10; is a statement
    Expressions can be a part of statements and calling a function is also an expression. */

   /* let y = {let x = 10; x+1}; /* Note that there is no semi colon after x+1 which makes it
    an expression. The output of x+1 here , it is 11 will be assigned to y */
    
    println!("The value of y: {}",y); */ 

    /* Example of function return */
/*    let z = sum(20,30);
    println!("The value of Z: {}",z); */

    // Conditional Statements
    /*let a = 10;
    if a < 100
    {
        println!("The value of a is less than 100");
    }
    else if a > 10
    {
        println!("The value of a is higher than 100");
    }*/
    // Assigning values through if statements
    /*
    let condition = true;

    let number = if condition {
        5}
        else{
            10
        };
    println!("The value of the number is : {}",number);
    */




}

fn func_a()
{
    println!("Func A");
}

fn func_b(x:i32, y:f64)
{
    println!("x = {}, y = {}",x, y);
}


    /* Returning vavlue from a function */
    /* ->i32 denotes that the function is returning int 32 */
    fn sum(x:i32,y:i32)->i32
    {
        let z = x+y;    // Note that there are no semi colon in the statement
        let z1 = z*1000;
        return z1;
    }