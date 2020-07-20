// Based on online book: The Rust Programming Language
// https://doc.rust-lang.org/book/title-page.html

// this tells Rust to load the contents of the module from another file with the same name as the module
mod typeutils;

mod chapter3 {
    use crate::typeutils;

    // You declare constants using the const keyword instead of the let keyword, and the type of the
    // value MUST be annotated:
    const CONST_FIVE: u32 = 5;

    pub(crate) fn common_concepts_variables() {
        println!();
        println!("3.1. Variables and Mutability");
        println!();

        // Variables are immutable by default
        let x = 5;
        // x = 6; // error
        println!("x = {}", x);

        // Mutability is opt-in:
        let mut mutx = 5;
        println!("mutx = {}", mutx);
        mutx = 6;
        println!("mutx = {}", mutx);

        // FIVE = 6; // runtime error: cannot assign to this expression
        println!("Constant = {}", CONST_FIVE);

        // Shadowing: You can declare a new variable with the same name as a previous variable,
        // and the new variable "shadows" the previous variable.
        let x = 5;
        let x = x + 1;
        let x = x * 2;
        println!("x = {}", x); // 12

        // By using let, we can perform a few transformations on a value but have the variable be
        // immutable after those transformations have been completed.
        // Because we’re effectively creating a new variable when we use the let keyword again,
        // we can change the type of the value but reuse the same name:
        let dashes = "---"; // String literals are slices, not proper strings! (see below)
        println!("dashes as &str: {}", dashes);
        let dashes = dashes.len();
        println!("shadowed dashes:  {}", dashes);

        // Note: String is not the same as &str
        // str is an immutable sequence of UTF-8 bytes of dynamic length somewhere in memory.
        // Since the size is unknown, one can only handle it behind a pointer.
        // This means that str most commonly appears as &str: a reference to some UTF-8 data,
        // normally called a "string slice" or just a "slice".

        // String is the dynamic heap string type, like Vec: use it when you need to own or modify
        // your string data. String keeps the buffer and is very practical to use.

        // &str is lightweight and should be used to "look" into strings.
        // You can search, split, parse, and even replace chunks without needing to allocate new memory.

        let s = String::from("hello world");
        let hello = &s[0..5];
        let world = &s[6..11];
        println!("string slices: {} {}", hello, world);

        let mut dashes: String = String::from("---");
        println!("dashes as String: {}", dashes);
        dashes.push('=');
        println!("dashes modified: {}", dashes); // ---=
        let str_slice = &dashes[1..];
        println!("slice 3..len: {}", str_slice);
        dashes.clear();
        println!("dashes cleared: {}", dashes);

        // String slices, as you might imagine, are specific to strings.
        // But there’s a more general slice type, too. Consider this array:

        let a = [1, 2, 3, 4, 5];
        let arr_slice = &a[1..3];
        println!("array slice: {:?}", arr_slice); // [2, 3]
    }

    pub(crate) fn common_concepts_scalar_data_types() {
        println!();
        println!("3.2.1 Scalar Data Types");
        println!();

        println!("Integers:");
        let i = 1000;
        println!("i = {}", i);
        println!("Type of i = {}", typeutils::type_of(&i));

        // let int_overflow: u8 = 300; // compile error

        println!("Floating Point:");
        let f = 2.0;
        println!("f = {}", f);
        println!("Type of f = {}", typeutils::type_of(&f));

        println!("Booleans:");
        let b = true;
        println!("b = {}", b);
        println!("Type of b = {}", typeutils::type_of(&b));

        println!("Characters:");
        let c = '😻';
        println!("c = {}", c);
        println!("Type of c = {}", typeutils::type_of(&c));
    }

    fn each_plus_one(arr: &mut [i32; 5]) {
        for i in 0..arr.len() {
            arr[i] = arr[i] + 1;
        }
    }

    pub(crate) fn common_concepts_compound_data_types() {
        println!();
        println!("3.2.2 Compound Data Types");
        println!();

        println!("Tuples:");
        // A tuple is a general way of grouping together a number of values with a variety of types
        // into one compound type. Tuples have a fixed length:
        // once declared, they cannot grow or shrink in size.
        let tup1 = (100, 1.4, 'z');
        println!("tup1 = {:?}", tup1);

        // To get the individual values out of a tuple, we can use pattern matching to destructure it:
        let (i, f, c) = tup1;
        println!("i = {}, f = {}, c = {}", i, f, c);
        // In addition to destructuring through pattern matching, we can access a tuple element directly
        // by using a period (.) followed by the index of the value we want to access:
        let tup1_1 = tup1.1;
        println!("tup1_1 = {}", tup1_1);

        let tup2: (i32, f64, u8) = (200, 2.4, 2); // with explicit type annotation
        println!("tup2 (pretty print) = {:#?}", tup2);

        println!();
        println!("Arrays:");
        // Arrays are useful when you want your data allocated on the stack rather than the heap,
        // or when you want to ensure you always have a fixed number of elements.
        // As opposed to Tuples, all values in an Array must have the same type.
        let arr1 = [1, 2, 3, 4, 5];
        println!("arr1 = {:?}", arr1);
        println!("Fist elem: {}", arr1[0]);
        println!("Last elem: {}", arr1[4]);
        // Runtime exception 'index out of bounds: the len is 5 but the index is 5'
        // println!("{}", arr1[5]);

        // Create an array that contains the same value for each element:
        let mut arr2 = [3; 5]; // same as let a = [3, 3, 3, 3, 3];
        println!("[3; 5] = {:?}", arr2);

        // iterating over arrays and modifying elements

        each_plus_one(&mut arr2);
        println!("After each_plus_one() = {:?}", arr2); //  [4, 4, 4, 4, 4];

        println!();
        println!("Vector (Note: not a Compound data type!)");

        // Note: vectors are the heap equivalent of arrays, they can grow or shrink in size:
        let mut v = vec![1, 2, 3, 4, 5];
        println!("v = {:?}", v);
        v.push(6);
        println!("v = {:?}", v);
    }

    pub(crate) fn common_concepts_control_flow() {
        println!();
        println!("3.4 Control Flow");
        println!();

        println!("if expressions:");
        let number = 12;
        if number % 4 == 0 { // good old modulo division trick
            println!("{} is divisible by 4", number);
        } else if number % 3 == 0 {
            println!("{} is divisible by 3", number);
        } else if number % 2 == 0 {
            println!("{} is divisible by 2", number);
        } else {
            println!("{} is not divisible by 4, 3, or 2", number);
        }

        // Because if is an expression, we can use it on the right side of a let statement:
        let condition = true;
        let i = if condition {
            5
        } else {
            6
        };
        println!("condition = {}, i = {}", condition, i);

        // Note there is no Ternary Conditional Operator in Rust! Must use if-expression as above.
        // syntax error:
        // let i = condition? 5 : 6;

        println!();
        println!("loops");
        // Loops are expressions too:
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2; // return something from the loop
            }
        };
        println!("The loop result is {}", result); // 20

        println!();
        println!("conditional loops:");

        let mut number = 3;
        while number != 0 {
            println!("{}", number);
            number -= 1;
        }
        println!("LIFTOFF!!!");

        println!();
        println!("looping through an array with 'for'");
        let mut arr = [10, 20, 30, 40, 50];

        for element in arr.iter() {
            println!("array elem value: {}", element);
        }

        for i in 0..arr.len() {
            println!("array elem {} = {}", i, arr[i]);
            arr[i] = arr[i] + 1;
            println!("  array elem {} = {}", i, arr[i]);
        }

        for number in 1..4 { // range
            println!("{}", number);
        }
        println!("LIFTOFF!!!");

        for number in (1..4).rev() { // reverse range
            println!("{}", number);
        }
        println!("LIFTOFF!!!");

        println!();
        println!("looping through a vector with 'for'");
        let v = vec![1, 2, 3, 4, 5];
        for element in v.iter() {
            println!("vector elem value: {}", element);
        }
    }
}

mod chapter4 {
    pub(crate) fn ownership_move_with_variables() {
        println!();
        println!("4. Ownership - Move - Variables");
        println!();

        // For primitive (scalar) types, an assignment is a copy, not a move,
        // as these are stack values, there is no heap memory to keep track of.
        // Stack values implement the Copy trait.

        let i = 42;
        let mut i_copy = i;
        i_copy += 1;
        println!("i = {}", i);
        println!("i_copy = {}", i_copy); // 43

        // However, for heap variables an assignment transfers ownership:

        let s = String::from("hello"); // s comes into scope
        let s2 = s; // s2 takes ownership of the value "hello"

        // Move is like a "shallow copy", meaning we copy the pointer, the length, and the capacity
        // that are on the stack. We do not copy the data on the heap that the pointer refers to.
        // But once you move ownership, you cannot use the old variable anymore:
        // println!("s = {}", s); // compile error: "value borrowed here after move"

        // allocated heap memory will be freed when s2 goes out of scope (s is ignored from now on)
        println!("s2 = {}", s2);

        // use clone() for "deep copy"
        let s2_clone = s2.clone() + " world!";
        println!("s2clone = {}", s2_clone);

        // you can still use the cloned variable as the clone is a separate copy, not a move
        println!("s2 still = {}", s2);
    }

    fn takes_ownership(some_string: String) { // some_string comes into scope
        println!("some_string = {}", some_string);
        // Here, some_string goes out of scope and `drop` is called. The backing memory is freed.
    }

    fn makes_copy(some_integer: i32) { // some_integer comes into scope
        println!("some_integer = {}", some_integer);
        // Here, some_integer goes out of scope. Nothing special happens.
    }

    fn gives_ownership() -> String {
        // gives_ownership will move its return value into the function that calls it
        let s = String::from("i came from a function"); // s comes into scope
        s
        // s is returned and moves out to the calling function
    }

    // takes_and_gives_back will take a String and return one
    fn takes_and_gives_back(a_string: String) -> String {
        // a_string comes into scope
        a_string
        // a_string is returned and moves out to the calling function
    }

    fn calculate_length_moved(s: String) -> (String, usize) {
        let length = s.len();
        (s, length)
    }

    pub(crate) fn ownership_move_with_functions() {
        println!();
        println!("4. Ownership - Move - Functions");
        println!();

        // The semantics for passing a value to a function are similar to those for assigning a value
        // to a variable. Passing a variable to a function will move (heap types) or copy (scalar types),
        // just as assignment does.

        let s = String::from("hello"); // s comes into scope

        // s's value moves into the function and so is no longer valid here
        takes_ownership(s);
        // compile error: "borrow of moved value: s"
        // println!("s = {}", s);

        let x = 5;  // x comes into scope
        makes_copy(x);
        // x would move into the function, but i32 implements Copy trait (stack value),
        // so it’s okay to still use x afterward
        println!("x = {}", x);

        // Returning values from functions can also transfer ownership:

        let s_that_came_from_a_function = gives_ownership(); // gives_ownership moves its return value into s2
        println!("s_that_came_from_a_function = {}", s_that_came_from_a_function);

        let s3 = String::from("hello");    // s3 comes into scope
        let s_passed_through_a_function = takes_and_gives_back(s3);
        // s3 is moved into takes_and_gives_back(), which also moves its return value into s4
        println!("s_passed_through_a_function = {}", s_passed_through_a_function);
        // println!("s3 = {}", s3); // value borrowed here after move

        // Taking ownership and then returning ownership with every function is a bit tedious.
        // What if we want to let a function use a value but not take ownership?
        // It’s quite annoying that anything we pass in also needs to be passed back if we want to
        // use it again, in addition to any data resulting from the body of the function that we
        // might want to return as well.
        // It’s possible to return multiple values using a tuple:

        let (s5, len) = calculate_length_moved(s_passed_through_a_function);
        println!("The length of '{}' is {}.", s5, len);
    }

    fn calculate_length(s: &String) -> usize {
        // s is a reference to a String
        s.len()
        // Here, s goes out of scope. But because it does not have ownership of what
        // it refers to, nothing happens.
    }

    fn try_to_change_borrowed_immutable(some_string: &String) {
        // Just as variables are immutable by default, so are references.
        // We’re not allowed to modify something we have a reference to.
        // Compile error: cannot borrow `*some_string` as mutable, as it is behind a `&` reference
        // some_string.push_str(", world");
        println!("some_string in change_borrowed() = {}", some_string);
    }

    fn change_borrowed_mutable(some_string: &mut String) {
        some_string.push_str(", world");
    }

    pub(crate) fn ownership_borrow() {
        println!();
        println!("4. Ownership - References and Borrowing");
        println!();

        // The Rules of References:
        // * At any given time, you can have either one mutable reference or any number of immutable references.
        // * References must always be valid.

        // The issue with the calculate_length_moved() function is that we have to return the String to
        // the calling function so we can still use the String after the call to calculate_length,
        // because the String was moved into calculate_length.
        // Here is how you would define and use a calculate_length function that has a reference to an
        // object as a parameter instead of taking ownership of the value:
        let hello_str = String::from("hello");
        // These ampersands are references, and they allow you to refer to some value without taking
        // ownership of it.
        let len = calculate_length(&hello_str);
        println!("The length of '{}' is {}.", hello_str, len);

        // Note: The opposite of referencing by using & is dereferencing, which is accomplished with
        // the dereference operator, * (Chapter 15)

        // We call having references as function parameters 'borrowing'.
        // So what happens if we try to modify something we’re borrowing? It doesn’t work!
        try_to_change_borrowed_immutable(&hello_str);

        // We can fix the error with just a small tweak:
        let mut hello_mut = String::from("hello");
        change_borrowed_mutable(&mut hello_mut);
        // First, we had to change s to be mut. Then we had to create a mutable reference with &mut s
        // and accept a mutable reference with some_string: &mut String.

        // But mutable references have one big restriction: you can have only one mutable reference to
        // a particular piece of data in a particular scope. This code will fail:
        let mut s_mut = String::from("hello");
        let r1 = &mut s_mut;
        // let r2 = &mut s; // error[E0499]: cannot borrow `s` as mutable more than once at a time
        println!("r1 = {}", r1);

        // The benefit of having this restriction is that Rust can prevent data races at compile time:
        // * Two or more pointers access the same data at the same time.
        // * At least one of the pointers is being used to write to the data.
        // * There’s no mechanism being used to synchronize access to the data.

        // We can use curly brackets to create a new scope, allowing for multiple mutable references,
        // just not simultaneous ones:
        let mut s2_mut = String::from("hello");
        {
            let r1 = &mut s2_mut;
            println!("r1 = {}", r1);
        } // r1 goes out of scope here, so we can make a new reference with no problems.
        let r2 = &mut s2_mut;
        println!("r2 = {}", r2);

        // warning: variable does not need to be mutable
        let mut s3 = String::from("hello");
        let r3 = &s3; // no problem
        let r4 = &s3; // no problem

        // We also cannot have a mutable reference while we have an immutable one:
        // Compile error[E0502]: cannot borrow `s3` as mutable because it is also borrowed as immutable
        // let r5 = &mut s3; // BIG PROBLEM

        println!("{}, {}", r3, r4);

        // any borrow must last for less than the scope of the owner
    }
}

mod chapter5 {
    use crate::typeutils;

    // Debug trait needed so we can print out values with {:?} and {:#?}
    #[derive(Debug)]
    struct User {
        // Note: if we wanted to use &str instead of String it would require the use of lifetimes (Chapter 10).
        // Lifetimes ensure that the data referenced by a struct is valid for as long as the struct is.
        username: String,
        email: String,
        sign_in_count: u64,
        active: bool,
    }

    fn build_user_using_field_init_shorthands(email: String, username: String, count: u64) -> User {
        User {
            username, // possible because the parameter name and the struct field name are exactly the same
            email, // possible because the parameter name and the struct field name are exactly the same
            sign_in_count: count,
            active: true,
        }
    }

    pub(crate) fn structs() {
        println!();
        println!("5. Structs");
        println!();

        // Structs and enums (discussed in Chapter 6) are the building blocks for creating new types in your
        // program’s domain to take full advantage of Rust’s compile time type checking.

        // create an instance of the struct
        let user1 = User {
            // We don’t have to specify the fields in the same order in which we declared them in the struct.
            email: String::from("someone@example.com"),
            username: String::from("someusername1"),
            active: true,
            sign_in_count: 1,
        };

        println!("user1: {:?}", user1);
        println!("user1 (pretty): {:#?}", user1);

        // create a mutable instance of the struct
        let mut user2 = build_user_using_field_init_shorthands(String::from("someone@example.com"),
                                                               String::from("someusername2"), 2);
        // To get a specific value from a struct, we can use dot notation
        user2.email = String::from("anotheremail@example.com"); // instance must be mutable to modify fields
        println!("user2: {:#?}", user2);

        // struct update syntax:
        // create a new instance of a struct that uses most of an old instance’s values but changes some
        let user3 = User {
            username: String::from("someusername3"),
            ..user1
        };
        println!("user3 (mostly copy of user1): {:#?}", user3);
    }

    pub(crate) fn tuple_structs() {
        println!();
        println!("5. Tuple Structs");
        println!();

        // You can also define structs that look similar to tuples, called tuple structs.
        // Tuple structs have the added meaning the struct name provides but don’t have names associated with their fields;
        // rather, they just have the types of the fields. Tuple structs are useful when you want to give the whole tuple
        // a name and make the tuple be a different type from other tuples, and naming each field as in a regular struct
        // would be verbose or redundant.

        #[derive(Debug)]
        struct Color(i32, i32, i32);

        #[derive(Debug)]
        struct Point(i32, i32, i32);

        let black = Color(0, 0, 0);
        println!("Color - black: {:?}", black);

        let origin = Point(0, 1, 2);
        println!("Point - origin: {:?}", origin);

        // Otherwise, tuple struct instances behave like tuples...
        // you can destructure them into their individual pieces:
        let Color(r, g, b) = black;
        println!("black destructured - r: {}, g: {}, b:{}", r, g, b);
        // and you can use a . followed by the index to access an individual value:
        println!("origin - x: {}", origin.0);
    }

    pub(crate) fn struct_newtypes() {
        println!();
        println!("5. Newtype Pattern");
        println!();

        // When a tuple struct has only one element, we call it newtype pattern. Because it helps to create a new type.

        struct Kilometers(u32);

        let distance = Kilometers(20);
        // Destructure the instance using a `let` binding
        let Kilometers(distance_in_km) = distance;
        println!("The distance: {} km", distance_in_km); // The distance: 20 km
        println!("Type of distance = {}", typeutils::type_of(&distance));
    }

    pub(crate) fn methods() {
        println!();
        println!("5. Methods");
        println!();

        // methods are different from functions in that they’re defined within the context of a struct
        // (or an enum or a trait object), and their first parameter is always self, which represents the instance of
        // the struct the method is being called on.

        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }

        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }

            fn perimeter(&self) -> u32 {
                (self.width + self.height) * 2
            }

            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }

            // If we wanted to change the instance that we’ve called the method on as part of what the method does,
            // we’d use &mut self as the first parameter.
            fn initialize(&mut self) {
                self.width = 0;
                self.height = 0;
            }

            // Associated Functions:
            // Se’re allowed to define functions within impl blocks that don’t take self as a parameter.
            // These are called associated functions because they’re associated with the struct. They’re still functions,
            // not methods, because they don’t have an instance of the struct to work with.
            // Associated Functions are useful as "factory methods" (example: String::from)
            fn square(size: u32) -> Rectangle {
                Rectangle {
                    width: size,
                    height: size,
                }
            }
        }

        // Each struct is allowed to have multiple impl blocks.
        // There’s no reason to separate these methods into multiple impl blocks in this case, but this is valid syntax.
        // Multiple impl blocks are useful for generic types and traits (chapter 10).
        impl Rectangle {
            fn transform_and_consume(self) -> (u32, u32) {
                // Having a method that takes ownership of the instance by using just self as the first parameter is rare;
                // this technique is usually used when the method transforms self into something else and you want to
                // prevent the caller from using the original instance after the transformation.
                return (self.width, self.height);
            }
        }

        let mut rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        let rect3 = Rectangle {
            width: 60,
            height: 45,
        };

        println!("rect1: {:?}", rect1);
        println!("The area of the rectangle is {} square pixels.", rect1.area());
        println!("The perimeter of the rectangle is {} pixels.", rect1.perimeter());

        println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
        println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

        rect1.initialize();
        println!("rect1 (initialized): {:?}", rect1);
        let (w, h) = rect1.transform_and_consume();
        println!("w: {}, h: {}", w, h);
        // println!("rect1: {:?}", rect1); // borrow of moved value: `rect1`

        let sq = Rectangle::square(3);
        println!("sq (created via Associated Function): {:?}", sq);
    }
}

mod chapter6 {
    use crate::typeutils;

    #[derive(Debug)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    // Like structs, enums can have methods too
    impl IpAddr {
        fn home() -> IpAddr {
            return IpAddr::V4(127, 0, 0, 1);
        }
    }

    fn route(ip_addr: IpAddr) {
        println!("Routing {:?}", ip_addr);
    }

    pub(crate) fn enums() {
        println!();
        println!("6. Enums");
        println!();

        // Enums are a feature in many languages, but their capabilities differ in each language.
        // Rust’s enums are most similar to algebraic data types in functional languages, such as F#, OCaml, and Haskell.

        let home = IpAddr::home();
        let loopback = IpAddr::V6(String::from("::1"));

        route(home);
        route(loopback);
    }

    pub(crate) fn option_enum() {
        println!();
        println!("6. Option Enum");
        println!();

        // Option is an enum defined by the standard library.
        // It encodes the very common scenario in which a value could be something or it could be nothing.
        // Expressing this concept in terms of the type system means the compiler can check whether you’ve handled all the
        // cases you should be handling; this functionality can prevent bugs that are extremely common in other
        // programming languages (e.g. Java's NullPointerException).
        // Rust doesn’t have the null feature that many other languages have. However, the concept that null is trying to
        // express is still a useful one: a null is a value that is currently invalid or absent for some reason.
        //    enum Option<T> {
        //      Some(T),
        //      None,
        //    }

        // we need to tell Rust what type of Option<T> we have, because the compiler can’t infer the type that the
        // Some variant will hold by looking only at a None value.
        let absent_number: Option<i32> = None;
        println!("type_of(absent_number): {}", typeutils::type_of(&absent_number));

        // So why is having Option<T> any better than having null?
        // In short, because Option<T> and T (where T can be any type) are different types, the compiler won’t let us
        // use an Option<T> value as if it were definitely a valid value.
        // When we have an Option<T> we have to worry about possibly not having a value, and the compiler will make sure
        // we handle that case before using the value.
        // In other words, you have to convert an Option<T> to a T before you can perform T operations with it.
        // Generally, this helps catch one of the most common issues with null: assuming that something isn’t null
        // when it actually is.

        let x: i8 = 4;
        let y: Option<i8> = Some(6);
        // error[E0277]: cannot add `std::option::Option<i8>` to `i8`
        // let sum = x + y;
        let sum = if y.is_none() { x } else { x + y.unwrap() };
        println!("Sum x + y = {}", sum);

        let sum = x + y.unwrap_or(0);
        println!("Sum (even more concise) = {}", sum);
    }

    #[derive(Debug)]
    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter,
    }

    fn value_in_cents(coin: &Coin) -> u8 {
        match coin {
            // A match arm has two parts: a pattern and some code
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter => 25,
        }
    }

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    pub(crate) fn pattern_matching() {
        println!();
        println!("6. Pattern matching");
        println!();

        // Rust has an extremely powerful control flow operator called match that allows you to compare a value against a
        // series of patterns and then execute code based on which pattern matches. Patterns can be made up of literal
        // values, variable names, wildcards, and many other things. The power of match comes from the expressiveness
        // of the patterns and the fact that the compiler confirms that all possible cases are handled.

        let coin = Coin::Dime;
        println!("value of {:?} = {} cents", coin, value_in_cents(&coin));

        // Matching with Option<T>
        let five = Some(5);
        let six = plus_one(five);
        println!("six: {:?}", six);
        let none = plus_one(None);
        println!("none: {:?}", none);

        // The _ placeholder
        // Rust also has a pattern we can use when we don’t want to list all possible values.
        let some_u8_value = 7u8;
        match some_u8_value {
            1 => println!("one"),
            3 => println!("three"),
            5 => println!("five"),
            7 => println!("seven"),
            _ => (),
        }

        // The if let syntax lets you combine if and let into a less verbose way to handle values that match one pattern
        // while ignoring the rest.
        // Consider this code that matches on an Option<u8> value but only wants to execute code if the value is 3.
        let some_u8_value = Some(0u8);
        match some_u8_value {
            Some(3) => println!("three"),
            _ => (),
        }
        // Instead, we could write this in a shorter way using if let.
        // You can think of if let as syntax sugar for a match that runs code when the value matches one pattern and
        // then ignores all other values.
        if let Some(3) = some_u8_value {
            println!("three");
        }
    }
}

mod chapter8 {
    pub(crate) fn collections() {
        println!();
        println!("8. Common Collections");
        println!();
    }
}

fn main() {
    println!();
    println!("Learning Rust from https://doc.rust-lang.org/book/title-page.html");

    chapter3::common_concepts_variables();
    chapter3::common_concepts_scalar_data_types();
    chapter3::common_concepts_compound_data_types();
    chapter3::common_concepts_control_flow();

    chapter4::ownership_move_with_variables();
    chapter4::ownership_move_with_functions();
    chapter4::ownership_borrow();

    chapter5::structs();
    chapter5::tuple_structs();
    chapter5::struct_newtypes();
    chapter5::methods();

    chapter6::enums();
    chapter6::option_enum();
    chapter6::pattern_matching();

    chapter8::collections();
}

