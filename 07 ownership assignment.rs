fn main(){

    let s1:String=String::from getstring(); // 3. s1 is the ownerof hello
    println!("this is s1 : {}",s1);

    let s2:String=String::from("world");            // 4. s2 is the owner of world
    let s3:String=String::from send_get_string(s2); // 5. transferring the ownership of s2 to send_get_string(s)
    println!("this is s3 : {}",s3);                 // 7. s2 is the new owner of world
}

fn getstring()->String{
    let new:String+String::from("hello"); // 1. new is the owner of hello
    return new;                           // 2. transfering the ownership
}

fn send_get_string(s:String)->String{
    return s;  // 6. transferring the ownership back to the caller
}



// Example 2

fn main(){
    let s1:String=String::from("hello");
    let (s2,len)=calculate_length(s1);
    println!("The length of '{}' is: {}",s2,len);
}
fn calculate_length(s:String) -> (String,usize) {
    let length= s.len();
    return (s,length);
}

/*
fn main(){
    let s1:String = String::from("hello");
    let len:usize = calculate_length(s1);
    println!("The length of {} is: {}",s1,len);  //this line will cause an error because s1 is moved
}
fn calculate_length(s:String) -> usize {
    s.len()
}
*/
