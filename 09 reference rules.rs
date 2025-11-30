/* borrowing rules:
    ✔ Many Immutable (&) — OK
    ✔ One Mutable (&mut) — OK
    ❌ Mutable + Immutable at same time — NOT OK
    ❌ Two Mutable at same time — NOT OK
*/

// example 1:
fn main(){
    let s1:String=String::from("hello");

    let r1=&s1;
    println!("{}",r1);

    let r2=&s1;
    println!("{}",r2);

    println!("{}, {}",r1,r2);
} 

//example 2:
fn main() {
    // Create a mutable String because we want to modify it.
    let mut s1: String = String::from("hello");

    // w1 is a mutable reference. It can READ and WRITE to s1.
    let w1 = &mut s1;
    w1.push_str(" world");  // modifying s1 through w1
    println!("{}", w1);     // OK: using w1

    // w2 is another mutable reference to the SAME variable (s1)
    // Rust allows creating it, but ONLY IF w1 is no longer used after this.
    let w2 = &mut s1;
    w2.push_str(" rust");  // modifying s1 through w2
    println!("{}", w2);    // OK: using w2

    // println!("{}", w1); 
    // ❌ ERROR: You cannot USE w1 after w2 exists.
    // Because that means two mutable references are ACTIVE at the same time.

    // println!("{}, {}", w1, w2);
    // ❌ ERROR: You cannot read from both w1 and w2 together.
}
