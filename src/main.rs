fn main() {
    fn print_string(s:String){
        println!("{}" ,s);
    }
    fn print_string_ref(s:&String){
        println!("{}" ,s);
    }
    fn change_string(s:&mut String){
        s.push_str("modified");
    }
    let my_originalstring= String::from("hello ");
 
    print_string(my_originalstring);  //ownership transferred to print_string if we try to use my_string after this our code would have a compile error

let a=6; // our integer  can be stored in stack
let y=String::from("ownership concept");// located in heap
let z=y; 
println!("a={}, z={}",a,z);
// the ownership transferred to integer z and y is no longer owner of the string

//borrowing concept
let string_two = String::from("borrow concept");//we intiallize a immutable reference 
print_string_ref(&string_two); // we cant modify our string but we can read the value 

let mut my_string_mut=String::from("hello"); 
change_string(&mut my_string_mut); //our change_String  uses a muteable reference so we can use the string without owning  
println!("{}",my_string_mut); //also because of we used mutable reference we also   modified our string but 

}

