fn main(){

    let hello:String=String::from("hello world");
    temp(hello);
    println!("In main: hello = {}", hello);  // this line will give error as ownership of hello is moved to temp function
}

fn temp(s:String){
    println!("In temp: s = {}", s);
}


/*  //this code will work fine stack data is copied not moved
fn main() {
    let num:u8=10;
    temp(num);
    println!("In main: num = {}", num);
}

fn temp(n:u8){
    println!("In temp: n = {}", n);
}
*/
