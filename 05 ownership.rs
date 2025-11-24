// 📌 OWNERSHIP RULES IN RUST
// 1️⃣ Every value has ONE owner.
// 2️⃣ When the owner goes out of scope, the value is dropped.
// 3️⃣ Ownership can be moved (transferred) but NOT copied by default.

fn main() {

    // str1 owns this String on the heap
    let str1 = String::from("Hello, world!");

    // Ownership of the String is MOVED from str1 to str2
    // After this line, str1 no longer owns the data
    // str1 becomes INVALID (cannot be used anymore)
    let str2 = str1;

    // ❌ ERROR: str1 was moved, so you cannot use it now
    // Rust prevents this to avoid double-free memory issues
    println!("{}", str1);

    // ✔ str2 is now the new owner of the String
    println!("{}", str2);
}
