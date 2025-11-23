fn main() {
    //String - Dynamic Length Strings -heap allocated
    //&str - Fixed Length Strings - stack allocated(special memory)

    let string:&str ="hello world";
    println!("{}",string);

    let mut string2:String = String::from("hello rust");
    println!("{}",string2);

    string2.push_str(" gm");//this not work in &str because &str is immutable
    println!("{}",string2);
}
