fn main() {


    // VARIABLES

    /*  
    This will not work as variables in Rust are non-mutable by default

    let x = 5;
    println!("The value of x is {x}");
    x = 6; // cannot assign twice to immutable variable
    println!("The value of x is {x}"); 
    
    */

    // we can however explicitly say that the variable is mutable, by adding "mut" keyword
    let mut x = 5;
    println!("The value of x is {x}");
    x = 6; // cannot assign twice to immutable variable
    println!("The value of x is {x}, because x is a mutable variable");  


    // CONSTANTS

    /* 
    You may think why the hell do I need constants if variables are already immutable.
    - constants CAN NOT be make mutable
    - constants are defined using const keyword
    - constants have to be type annotated
    - constants can be defined in any scope, including global
    */

    const THREE_HOURS_IN_SECONDS: u32 = 3 * 60 * 60;


    // SHADOWING

    /* 
    Variables can be shadowed just by being redefined with let keyboard, also fun - subscopes can be created with curly brackets.
    Using shadowing instead of mut is also helpful when keeping the same variable name between different types of the variable.
    Error will be raised if we change the type with mut.
    */

    let x = 5;
    
    let x = x + 1;

    {
        let x = x * 2;
        println!("Value of x in the subscope is {x}");
    }

    println!("Value of x in main scope after being shadowed is {x}");

    // Example of changing the type
    let spaces = "   ";

    println!("The number of spaces is {spaces}, better get a meter");

    let spaces = spaces.len();

    println!("The number of spaces is {spaces}");

    
}