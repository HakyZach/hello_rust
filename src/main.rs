fn main() {
  
    println!("Hello, world!");

//basic input and output

    let mut first = String::new();
    println!("Please enter your first name:");
    std::io::stdin().read_line(&mut first).unwrap();

    let mut last = String::new();
    println!("Now please enter your last name:");
    std::io::stdin().read_line(&mut last).unwrap();

    println!("Welcome, {} {}", first,last);

    //conditionals
    let number1 = 28;
    let number2 = 41;
    if number1 > number2{
        println!("number 1 is greater");
    }
    else if number2 > number1{
        println!("number 2 is greater");
    }
    else{
        println!("they are equal");
    }
    


    //for loop in rust
    for x in 1..16{

        //checks to see if the counter is on an even number
        if x % 2  == 0{
            println!("{} is even",x);
            
        }
        else{
            println!("{} is odd",x)
        }
    }
}
