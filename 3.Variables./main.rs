fn main(){
    println!("hello");


// To create a variable in Rust, use the let keyword and specify the name of the variable.
let name="Ayukt";
println!("My name is: {}",name);

// What is {}?
// Rust uses {} as a placeholder in println!() to show variable values.

let name="Ayukt";
let age="18";
println!("{} is {} years old boy.",name ,age); //name and age should be in order.

// Variable Values Cannot be Changed by Default
let x=5;
// x=10;
println!("{}",x);

// Change Variable Values.
// If you want to change the value of a variable, you must use the mut keyword (which means mutable/changeable):

let mut x = 5;
println!("Before:{}",x);
x=10;
println!("After:{}",x);


hi,hello.        




}
