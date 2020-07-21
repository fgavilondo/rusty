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

pub(crate) fn plain_structs() {
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

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Methods are different from functions in that they’re defined within the context of a struct
// (or an enum or a trait object), and their first parameter is always self, which represents the instance of
// the struct the method is being called on.

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

pub(crate) fn methods() {
    println!();
    println!("5. Methods");
    println!();


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

// Unit tests go in the same files as the code

#[cfg(test)]
mod tests {
    // The tests module is a regular module that follows the usual visibility rules.
    // Because the tests module is an inner module, we need to bring the code under test in the outer module into the
    // scope of the inner module.
    // We use a glob here so anything we define in the outer module is available to this tests module.
    use super::*;

    #[test]
    fn larger_rectangle_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_rectangle_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
