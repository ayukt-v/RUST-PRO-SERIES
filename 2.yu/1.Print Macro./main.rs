fn main(){
    println!("hi");


/*There is also a print!() macro, which is similar to println!().
The only difference is that it does not insert a new line at the end of the output:*/

print!("hi");
print!("hello\n");

// If you really want to add a new line in print!(), you can use the \n character:

print!("hi\n");
print!("hello\n");
print!("bye\n");
print!("I will      print on      the same line.\n"); //like a pre tag in HTML.



// You can also break up a line in the middle of a sentence. This goes for both print!() and println!():
println!("Hello World!\nThis line was broken up!");
print!("Hello World!\nThis line was broken up!\n");
}