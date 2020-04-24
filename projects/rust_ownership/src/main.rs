fn main() {
    /*
    println!("Hello, world!");
    /* In this we see how the memory is allocated
    in the Rust Environment 
    Generally, the memory is allocated in Stack and Heap
    where the static variables are stored in Stack Memory 
    and dynamic memory or run time memory allocation is done in heap

    For Dynamic Memory allocation the programmer has to allocate in 
    other programming language and have to periodically clear the 
    memory allocated 

    In Java, there is something called as Garbage Collector

    In Rust, there is a new concept called ownership 

    Ownership is a new way here a value has only one onwership
    */
    {
        let s = "hello";

        println!("s = {}",s);
    }
    //println!("s = {}",s); // This throws an error as variable s is out of scope

    /* MEMORY ALLOCATION */
    /* In Rust, memory is automatically returned once the variable goes 
    out of scope. */

    {
        let s = String::from("Hello");
        /* Here "Hello" is storeed in the heap memory
        and during run time the string is allocated 
        to variable s */
        //Once the scope is over, s is no longer valid
    }

    let x = 10;
    let y = x; // Here a copy is created and y gets value 10
    println!("x = {}, y = {}",x,y);
    // Here, the ownership of the value x and y are different
    {
        let mut s1 = String::from("Hello");
        s1.push_str(", World!");
        
       // let s2 = s1; // Here the ownership of s1 is transferred to s2
      //  println!("s2 = {}",s2);
      // println!("s1 = {}",s1); // s1 no longer holds the value and errors
        // To solve this we use clone
        let s2 = s1.clone(); // Here the ownership of s1 is transferred to s2
        println!("s2 = {}",s2);
        println!("s1 = {}",s1);
    }
    */
    /* Referencing a string 
    {
        let s = String::from("Hello");
        takes_ownership(s);   // referencing the value
        println!("The string is {}",s);  // Now the ownershipis given. 
        // This will give an error
    }
    */
    /* return values and scope */
    {
        let s1 = gives_ownership();
        println!("the string is {}",s1);
    

        /* find length */
        let l = get_length(&s1);
        println!("The string length: {}",l);
        println!("the string is {}",s1);
    }
}
fn takes_ownership(s:String)   // passing by value
{
    println!("the string is {}",s);
}
fn gives_ownership() ->String{
    let s = String::from("Hello World");
    s
}
fn get_length(s:&String)->usize  // Call by Reference 
{
    let len = s.len();
    len
}