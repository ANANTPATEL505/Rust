fn main() {
  let num1=10;
  let num2=20;
  let sum=add(num1,num2);
  println!("{}",sum);
}

fn add(a:u8,b:u8)->u8{  //->u8 indicates the return type
    return a+b;
}
