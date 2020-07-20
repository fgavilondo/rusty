// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//
//     for &item in list {
//         // binary operation `>` cannot be applied to type `T`. `T` might need a bound for `std::cmp::PartialOrd`
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct MultiTypePoint<T, U> {
    x: T,
    y: U,
}

pub(crate) fn generic_types() {
    println!();
    println!("10. Generic Types");
    println!();

    // Rust implements generics in such a way that your code doesn’t run any slower using generic types than
    // it would with concrete types. Monomorphization is the process of turning generic code into specific code by
    // filling in the concrete types that are used when compiled.

    // Generics in Function Definitions

    let number_list = vec![34, 50, 25, 100, 65];
    // let result = largest(&number_list);
    // println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    // let result = largest(&char_list);
    // println!("The largest char is {}", result);

    // Generics in Struct Definitions

    let integer = Point { x: 5, y: 10 };
    println!("integer point {:?}", integer);
    let float = Point { x: 1.0, y: 4.0 };
    println!("float point {:?}", float);
    // error: mismatched types
    // let wont_work = Point { x: 5, y: 4.0 };

    // To define a Point struct where x and y are both generics but could have different types, we can use
    // multiple generic type parameters, You can use as many generic type parameters in a definition as you want,
    // but using more than a few makes your code hard to read. When you need lots of generic types in your code,
    // it could indicate that your code needs restructuring into smaller pieces.
    let multi = MultiTypePoint { x: 5, y: 4.0 };
    println!("multi point {:?}", multi);

    // Generics in Enum Definitions

    #[derive(Debug)]
    enum MyResult<T, E> {
        Ok(T),
        Err(E),
    }

    let mut res: MyResult<&str, &str> = MyResult::Ok("everything Ok");
    println!("result {:?}", res);
    res = MyResult::Err("Kaboom!");
    println!("result {:?}", res);

    // Generics  in Method Definitions

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x());

    // We could implement methods only on Point<f64> instances rather than on Point<T> instances with any generic type.
    // Here we use the concrete type f64, meaning we don’t declare any types after impl:

    impl Point<f64> {
        fn distance_from_origin(&self) -> f64 {
            (self.x.powi(2) + self.y.powi(2)).sqrt()
        }
    }

    let pf64 = Point { x: 5.0, y: 10.0 };
    let distance = pf64.distance_from_origin();
    println!("distance = {}", distance);
}

pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

pub(crate) fn traits() {
    println!();
    println!("10. Traits");
    println!();

    // A trait tells the Rust compiler about functionality a particular type has and can share with other types.
    // We can use traits to define shared behavior in an abstract way.
    // We can use trait bounds to specify that a generic can be any type that has certain behavior.

    // A type’s behavior consists of the methods we can call on that type. Different types share the same behavior if
    // we can call the same methods on all of those types. Trait definitions are a way to group method signatures
    // together to define a set of behaviors necessary to accomplish some purpose.

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: '{}'", tweet.summarize());
}