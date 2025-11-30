// References and Borrowing in Rust

// Example 1: Immutable References readonly
fn main(){
    let mut s1:String=String::from("hello");
    let len=calculate_length(&s1); 
    println!("The length of '{}' is {}.",s1,len);
}

fn calculate_length(s:&String)->usize{
    s.len()
}



// Example 2: Mutable References writable
fn main(){
    let mut s2:String=String::from("hello"); // s2 is mutable
    append_world(&mut s2); // pass a mutable reference to s2
    println!("{}",s2); 
}

fn append_world(s:&mut String){ // s is a mutable reference
    s.push_str(" world"); 
}
