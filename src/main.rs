use std::io;

fn main() {

    /* Welcome the user and explain the program */
    println!("\n Hello! Welcome to Equals-1000. This program will
first ask you to select either addition, subtraction,
multiplication, or division. The program will then ask
you for two numbers 'A' and 'B'. The program will then
perform the chosen mathematical expression on the two
numbers. The solution will then replace 'A' and the
expression will be performed continuously until the answer
exceeds the absolute value of 1,000. It will then inform
you how many times the expression needed to be performed");

    /* Declare some mutable and immutable variables */
    let mut continue_ = String::new();
    let stop_criteria = "n".to_string();
    let goal = 1000;
    let mut math_type = String::new();

    /* Loop until the user does not want to enter another
    number */
    while continue_.trim() != stop_criteria {

        let mut a = 5;
        let mut b = 4;

        /* Empty variables */
        continue_ = String::new();
        math_type = String::new();
    
        /* Create an immutable variable that contains the target
        number for the mathematical expressions */
        println!("\n The unchangeable goal is {goal}!");

        /* See what math the user would like to perform */
        println!("\n What math would you like to do? (+,-,*,/)");
        match io::stdin().read_line(&mut math_type) {
            Ok(_) => println!(""),
            Err(e) => println!("Oops. {e}")
        }

        /* Get numbers */
        

        /* Do some math! */
        if math_type.trim() == "+" {
            println!("You picked +");
        } else if math_type.trim() == "-" {
            println!("You picked -");
        } else if math_type.trim() == "*" {
            println!("You picked *");
        } else if math_type.trim() == "/" {
            println!("You picked /");
        } else {
            print!("That's no good!")
        }

        /* Demonstrate function */
        calculate_add(a, b, goal);

        /* Check if the user wants to do another round */
        println!("\n Would you like to go again? y/n");
        match io::stdin().read_line(&mut continue_) {
            Ok(_) => println!(""),
            Err(e) => println!("Oops. {e}")
        }

    };

    /* Say goodbye */
    println!("\n Have a good day!");

}


/* Create a seperate function to sum two numbers */
fn calculate_add(a: i32, b: i32, goal: i32) {

    let mut _counter = 0;
    let mut c = a + b;

    while c < goal.abs() {
        c += b;
        _counter += 1;
    }

    println!("That took {_counter} iterations to exceed the absolute value of 1000!");
    
}


/* A seperate function to subtract two numbers */
fn calculate_sub(a: i32, b: i32, goal: i32) {

    let mut _counter = 0;
    let mut c = a - b;

    while c < goal.abs() {
        c -= b;
        _counter += 1;
    }

    println!("That took {_counter} iterations to exceed the absolute value of 1000!");
    
}


/* A seperate function to sum two numbers */
fn calculate_mult(a: i32, b: i32, goal: i32) {

    let mut _counter = 0;
    let mut c = a * b;

    while c < goal.abs() {
        c *= b;
        _counter += 1;
    }

    println!("That took {_counter} iterations to exceed the absolute value of 1000!");
    
}


/* A seperate function to sum two numbers */
fn calculate_div(a: i32, b: i32, goal: i32) {

    let mut _counter = 0;
    let mut c = a / b;

    while c < goal.abs() {
        c /= b;
        _counter += 1;
    }

    println!("That took {_counter} iterations to exceed the absolute value of 1000!");
    
}