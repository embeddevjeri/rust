fn main() {
    /* Different Variable and Mut keyword */
    // assign multiple values

    // let (mut i1, i2) = (10,20);
    // println!("i1 = {}, i2 = {}",i1,i2);

    // let f1:f64 = 5.26;
    // let f2:f32 = 10.548645;
    // println!("f1 = {}, f2 = {}",f1,f2);

    // let b1:bool = true;
    // let b2 = false;
    // let b3 = 50<20;

    // println!("b3 = {}",b3);

    // let c1:char = 'a';
    // let c2:char = '\u{1F601}';
    // println!("c1 = {}, c2 = {}",c1,c2);

 
    /* Tuples and Arrays */
    let tup:(i32,u8,f32) = (10,20,2.5); //Creating a tuple
    let(x,y,z) = tup;   // Destructuring
    println!("x = {}, y= {}, z = {}",x,y,z);
    let i = tup.0;
    let u = tup.1;
    let f = tup.2;
    println!("i = {}, u= {}, f = {}",i,u,f);
    let arr = [10,20,30,40,50]; // creating arrays
    /* Arrays are generally used in to store the values 
    in the stack memory rather than in heap memory */ 
    println!("First Element: {}",arr[0]);
    println!("Second Element: {}",arr[1]);





}
