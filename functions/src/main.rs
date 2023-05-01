fn fib(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        2 => 1,
        _ => fib(n-1) + fib(n-2),
    }
}

fn if_else_assignment() {
    let condition = true;
    let condition2 = false;
    let number = if condition { 5 } else if condition2 {7}  else { 6 };
    println!("number is: {number}");
}


fn get_first_word(text: String) -> String {
    let mut word = String::from("");
    for ch in text.chars() {
        if ch == ' ' {
            break;
        }
        word += &String::from(ch);
    }
    return word;
}


struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle{
    fn area(&self) -> u32 {
        self.width * self.height
    }
}


fn main() {
 let n = 7;
 let result = fib(n);
 println!("fib at {n} is {result}");

 if_else_assignment();

 let word = get_first_word("hello world".to_string());
 println!("{word}");

 let rect = Rectangle {
    width: 10,
    height: 5,
 };
 println!("{}", rect.area());
}
