pub fn run() {
    let str1 = String::from("World");
    // let str2 = str1; // This will cause a move of the value to the varible
    let str2 = str1.clone(); // This way the value will be cloned and mainteined in both variables
    println!("Hello {}", str1);
    println!("Hello {}", str2);
}
