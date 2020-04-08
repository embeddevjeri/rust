fn main() {
    println!("Hello, world!");
    /*let mut count = 0;
    /* loop statement keeps on executing the code 
    unless a break statemnt is encountered */
    loop
    {
        count = count +1 ;
        println!("Hello grinntech");
        if (count >10)
        {
            break;
        }
    }
    */
    // while loop
    /*
    let mut number = 5;
    while number != 0
    {
        println!("number = {}",number);
        number -= 1;
    }
    */
    // For loop
    /* FOR loop will grant more data safety
    as iter() is used to make sure all the 
    elements are gone through where as in while loop
    there could be a chance of otubound statements
    
    Execution of for loop is faser compared to while loop
    as while loop are executed in the run time*/
    let a:[i32;5] = [10,20,30,40,50];
    for element in a.iter()
    {
        println!("element = {}",element);
    }
    // While loop
    let mut index = 0;
    while index < 5
    {
        println!("element = {}..",a[index]);
        index += 1;
    }

}
