pub fn run() {
    let str1 = String::from("World");
    // let str2 = str1; // This will cause a move of the value to the varible
    let str2 = str1.clone(); // This way the value will be cloned and mainteined in both variables
    let str3 = print_return_str(str1); 

    println!("str3 = {}", str3);
}

fn print_str(x: String) {
    println!("A string {}", x);
}

fn print_return_str(x: String) -> String {
    println!("A string {}", x);
    x
}

fn change_string(name: &mut String) {
    name.push_str(" is happy");
    println!("Message: {}", name);
}
