1. Statically typed language ⇒ data types for all variables must be known at compile time.
2. Variables are immutable by default. Use `mut` keyword to demarcate mutability.
3. Variables can be shadowed.
4. Constants are always immutable.
5. **Scalar types** in Rust: `integer`, `floating-point`, `numbers`, `booleans` and `characters`.
6. Integer overflow behvaiour is similar to other languages. However, there will be some help from the compiler during debug mode.

```rust
To explicitly handle the possibility of overflow, you can use these families of methods provided by the standard library for primitive numeric types:

* Wrap in all modes with the wrapping_* methods, such as wrapping_add.
* Return the None value if there is overflow with the checked_* methods.
* Return the value and a boolean indicating whether there was overflow with the overflowing_* methods.
* Saturate at the value’s minimum or maximum values with the saturating_* methods.
```

7. Integer types - u8/16/32/64/128/size, i8/16/32/64/128/size. i32 is default.
8. f32 and f64 are floating point numbers. 
9. Operators in Rust - [https://doc.rust-lang.org/book/appendix-02-operators.html](https://doc.rust-lang.org/book/appendix-02-operators.html)
10. `true` and `false` are the `bool` types.
11. `char` is 4 bytes in size and represents a Unicode Scalar Value and is declared using single quotes.

```rust
// example of mutability
let mut x = 5;
x = x + 1;

// example of shadowing
let spaces = "    ";
let spaces = spaces.len(); // type of this variable is different from original.
// or
let x = 1;
{
	let x = 2;
	println!("x is: {x}");
}
println!("x is: {x}");

const SECONDS_IN_HOUR = 60 * 60; // this value will be comuted at compile time.

let x = 2.0; //f64
let y: f32 = 2.0; //f32

let t = true;
let f: bool = false;

let c = 'z'; let z: char = '😻';
```

12. **Compound types: `tuples` and `arrays`**
13. Tuples have fixed lengths and can not grow or shrink is size. Each position in a tuple has a type.

```rust
let tup: (i8, f64, bool) = (127, 2.0, false)
let tupBool = tup.2;

let student = ("Sam", 5 , 1);
let (name, class, roll_num) = student; // Destructuring
```

14. `unit` is a special value and type in Rust. An empty tuple is written like `()`. Expressions implicitly return the unit value if they don’t return any other value.
15. Array have a fixed length and every element of an array must have the same type.

```rust
let a = [1, 2, 3, 4];
let a: [i32; 3] = [1,2,3];
let a = [0; 2]; // [0, 0] initializes the array with same value 2 times.
let first = a[0];
let last = a[2]; // this will cause panic as a.len() == 2
```

16. Function declaration order doesn’t matter as long as the function is declared in the scope.

```rust
fn my_function(param1: i32, param2: char) { // type annotations are required
	println!("My inputs: {param1}, {param2}");
}
```

17. **Statements** are instructions that perform an action and do not return a value. **Expressions** evaluate to a resultant value.

<aside>
💡 Expressions do not end in semicolon. Statements end in semicolon.

</aside>

```rust
let y = { let x = 3; x + 1 }; 
// this will set y to 4. Note there's no ; at the end of x+1
```

18. Rust functions implicitly return the last expression in the block of the function body.

```rust
fn number() -> u32 {
	1
} 
// returns 1 as it's the last expression. 
// You can use return to return early.
// Note: there's no ; at the last expression.
fn plus_one(x: i32) -> i32 {
 x + 1
}
```

19. `if`, `else if` and `else`.

```rust
let condition = true;
let condition2 = false;
let number = if condition { 5 } else if condition2 {7}  else { 6 };
```

20. `loop`, `while` and `for`. for loops are more used.
21. `break` and `continue`

```rust
fn loopExample() {
    let mut count = 0;
    'counting_up: loop { // we are using a loop label here.
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

let mut number = 2;
while number != 0 {
	number -= 1;
}

let sum = 0;
for num in (1..5).rev(){
	sum += num;
}
```

22. `Ownership` is Rust’s most unique feature and has deep implications for the rest of the language. It enables Rust to make memory safety guarantees without needing a garbage collector.
    - All programs have to manage the way they use a computer’s memory while running. Some languages have garbage collection that regularly looks for no-longer-used memory as the program runs; in other languages, the programmer must explicitly allocate and free the memory. Rust uses a third approach: memory is managed through a system of ownership with a set of rules that the compiler checks. If any of the rules are violated, the program won’t compile.
23. Stack and Heap:
    - Adding data is called *pushing onto the stack*, and removing data is called *popping off the stack*. All data stored on the stack must have a known, fixed size. Data with an unknown size at compile time or a size that might change must be stored on the heap instead.
    - The heap is less organized: when you put data on the heap, you request a certain amount of space. The memory allocator finds an empty spot in the heap that is big enough, marks it as being in use, and returns a *pointer*, which is the address of that location. This process is called *allocating on the heap* and is sometimes abbreviated as just *allocating.*
24. Stack access is faster as it is the top element. Heap access is slower as it requires following the pointer.
25. `**Ownership rules`:**
    1. Each value in Rust has a owner
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
26. Ownership chapter is worth revisiting - [https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html). 

```rust
let s1 = String::from("hello");
let s2 = s1; // shallow copy
println!("{}", s1); // throws compile time error as s1 is being referrenced (value borrowed) after move
```

27. Functions and assignments of variables that are allocated on heap results in ownership transfer.
28. You can pass by reference using `&`. Act of creating a reference is called borrowing.

```rust
fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

29. `Mutable` references have one big restriction: if you have a mutable reference to a value, you can have no other references to that value. This code that attempts to create two mutable references to `s` will fail:

```rust
let mut s = String::from("hello");

let r1 = &mut s;
let r2 = &mut s; // This will fail at compile time.

let r3 = &s;
let r4 = &s; // this is okay as it's unmutable reference.
// BIG PROBLEM, because we are can not have a mutable reference
// if we have an immutable reference to the same value.
let r5 = &mut s;

println!("{}, {}", r1, r2, r3, r4, r5);
```

```rust
// This works as the immutables have gone out of scope when we declare a 
// mutable refernce.
let mut s = String::from("hello");

let r1 = &s; // no problem
let r2 = &s; // no problem
println!("{} and {}", r1, r2);
// variables r1 and r2 will not be used after this point

let r3 = &mut s; // no problem
println!("{}", r3);
```

30. Note how ownership and borrowing are different.

```rust
fn dangle() -> &String { // dangle returns a reference to a String

    let s = String::from("hello"); // s is a new String

    &s // we return a reference to the String, s
} // Here, s goes out of scope, and is dropped. Its memory goes away.
  // Danger!

fn no_dangle() -> String {
    let s = String::from("hello");

    s
} // we are moving ownership out, and nothing is deallocated.
```

31. Things to remember:
    - At any given time, you can have *either* one mutable reference *or* any number of immutable references.
    - References must always be valid.
32. `Slices` in rust: *Slices* let you reference a contiguous sequence of elements in a collection rather than the whole collection. A slice is a kind of reference, so it does not have ownership.

```rust
let s = String::from("hello");

let slice = &s[0..2];
let slice = &s[..2];
let slice = &s[3..];

let a = [1, 2, 3, 4, 5];
let slice = &a[1..3];
assert_eq!(slice, &[2, 3]);
```

33. `struct` is just like other languages but it is also immutable by default.

```rust
struct User {
    active: bool, // Implements Copy trait
    username: String,
    email: String,
    sign_in_count: u64, // Implements Copy trait
}
let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
};
user1.email = String::from("anotheremail@example.com");
```

34. Note that the entire instance must be mutable; Rust doesn’t allow us to mark only certain fields as mutable.
35. Struct also have javascript like Field Init shorthand.

```rust
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
```

36. It also has javascript like de-structuring but it uses move.

```rust
let user2 = User {
    active: user1.active,
    username: "username2",
    email: String::from("another@example.com"),
    sign_in_count: user1.sign_in_count,
};
// user1 is still valid here as we have only used attributes that implement Copy trait

let user2 = User {
    email: String::from("another@example.com"),
    ..user1
};
// user1 can no longer be used here as it's contents are now moved
```

37. `Tuple structs` are structs that are named tuples.
    
    ```rust
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    ```
    
38. `Unit-like structs` can be used when you need to implement a trait but don’t have any data to store.
    
    ```rust
    struct AlwaysEqual;
    let subject = AlwaysEqual;
    ```
    
39. `#[derive(Debug)]` for printing a struct
    
    ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    fn main() {
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
    
        println!("rect1 is {:?}", rect1); // You can also use {:#?}
    }
    ```
    
40. `dbg!(&rect1)`returns ownership of the expression’s value so we can do
    
    ```rust
    #[derive(Debug)]
    struct Rectangle {
        width: u32,
        height: u32,
    }
    
    impl Rectangle {
    		fn area(&self) -> u32 {
            self.width * self.height
        }
    }
    
    fn main() {
        let scale = 2;
        let rect1 = Rectangle {
            width: dbg!(30 * scale), // width will get the value of the expression
            height: 50,
        };
    
        dbg!(&rect1); // pass by reference to avoid giving ownership to dbg! macro
    }
    ```
    
41. **Associated functions,** all functions defined within an `impl` block are associated to the type name after the `impl`. You can define functions inside `impl` block that don’t have `self` as their first parameter if they don’t need an instance of the type to work with. 
    
    ```rust
    impl Rectangle {
        fn square(size: u32) -> Self {
            Self {
                width: size,
                height: size,
            }
        }
    }
    fn main {
    	let sq = Rectangle::square(3); // :: syntax is used for both associated functions and namespaces created by modules.
    }
    ```
    
42. `enum` : you can put any kind of data inside an enum variant: strings, numeric types, or structs, another enum, etc.
    
    
    ```rust
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    ```
    
    ```rust
    struct Ipv4Addr {
        // --snip--
    }
    
    struct Ipv6Addr {
        // --snip--
    }
    
    enum IpAddr {
        V4(Ipv4Addr),
        V6(Ipv6Addr),
    }
    ```
    
43. You can also define methods on enums similar to structs
    
    ```rust
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }
    impl Message {
        fn call(&self) {
            // method body would be defined here
        }
    }
    let m = Message::Write(String::from("hello"));
    m.call();
    ```
    
44. `Option enum`. For `absent_number`, Rust requires us to annotate the overall `Option` type: the compiler can’t infer the type that the corresponding `Some` variant will hold by looking only at a `None` value. Here, we tell Rust that we mean for `absent_number` to be of type `Option<i32>`.
    
    ```rust
    enum Option<T> {
        None,
        Some(T),
    }
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    ```
    
45. `match` pattern matching based arm selection. Matches have to be exhaustive else compiler error.
    
    ```rust
    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }
    
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
    ```
    
46. You can use `other` or `_` in matching patterns as a catch all.
47. `if let` is a syntactic sugar on top of `match` clause. It can be useful when you don’t want to match all possibilities. You can also use else along with if let.
    
    ```rust
    let mut count = 0;
    if let Coin::Quarter(state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        count += 1;
    }
    ```
