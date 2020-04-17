fn main() {
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
        println!("s1 = {}",s1);
        let s2 = s1; // Here the ownership of s1 is transferred to s2
        println!("s2 = {}",s2);
      // println!("s1 = {}",s1); // s1 no longer holds the value and errors
    }

    

   
    
    
}
