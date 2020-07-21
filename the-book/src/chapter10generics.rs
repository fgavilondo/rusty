use std::fmt;

// Generics in Function Definitions
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list {
        // binary operation `>` cannot be applied to type `T`. `T` might need a bound for `std::cmp::PartialOrd`
        if item > largest {
            largest = item;
        }
    }

    largest
}

// Generics in Struct Definitions

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
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);

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

// The Summary trait needs to be public if we wanted other crates to implement it.
pub trait Summary {
    fn summarize(&self) -> String;
}

// Note: we can use the pub keyword to decide which modules, types, functions, and methods in our code should be public,
// and by default everything else is private.
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    // Trait methods share the visibility of the trait (Summary), so `pub` is unnecessary (and is a compile error)
    fn summarize(&self) -> String {
        format!("{}", self.content)
    }
}

impl fmt::Display for Tweet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tweet from {}", self.username)
    }
}

// Traits as Parameters:
// Instead of a concrete type for the item parameter, we specify the impl keyword and the trait name. This parameter
// accepts any type that implements the specified trait.
// In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize.

// Trait Bound Syntax
pub fn breaking_news<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

// The impl Trait syntax is convenient and makes for more concise code in simple cases.
pub fn breaking_news_syntax_sugar(item: &impl Summary) {
    println!("Breaking news (syntax sugar): {}", item.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
pub fn notify(item: &(impl Summary + fmt::Display)) {
    println!("New {}: {}", item, item.summarize());
}

// Clearer Trait Bounds with where Clauses:
// Using too many trait bounds has its downsides. Each generic has its own trait bounds, so functions with multiple
// generic type parameters can contain lots of trait bound information between the function’s name and its parameter
// list, making the function signature hard to read. For this reason, Rust has alternate syntax for specifying
// trait bounds inside a where clause after the function signature. So instead of writing this:

fn some_function<T: fmt::Display + Clone, U: Clone + fmt::Debug>(t: &T, u: &U) -> i32 {
    return 1;
}

// we can use a where clause, like this:

fn some_function_with_where_clause<T, U>(t: &T, u: &U) -> i32
    where T: fmt::Display + Clone,
          U: Clone + fmt::Debug
{
    return 1;
}

// Returning Types that Implement Traits

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

// Traits with default implementation

pub trait SummaryWithDefaultImplementation {
    fn summarize_author(&self) -> String;

    // Default implementations can call other methods in the same trait, even if those other methods don’t have a
    // default implementation. In this way, a trait can provide a lot of useful functionality and only require
    // implementors to specify a small part of it.
    fn summarize(&self) -> String {
        format!("(Read more from {}...)", self.summarize_author())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl SummaryWithDefaultImplementation for NewsArticle {
    fn summarize_author(&self) -> String {
        format!("@{}", self.author)
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

    // Summary trait
    println!("1 new tweet: '{}'", tweet.summarize());

    // fmt::Display trait
    println!("tweet display: '{}'", tweet);

    // Traits as Parameters
    breaking_news(&tweet);
    breaking_news_syntax_sugar(&tweet);

    // Specifying Multiple Trait Bounds with the "+" Syntax
    notify(&tweet);

    let article = NewsArticle {
        headline: String::from("Penguins win the Stanley Cup Championship!"),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
             hockey team in the NHL.",
        ),
    };

    // trait with default implementation
    println!("New article available! {}", article.summarize());

    // Using Trait Bounds to Conditionally Implement Methods
    // TODO
}

pub(crate) fn lifetimes() {
    println!();
    println!("10. Lifetimes");
    println!();

    // Another kind of generic is called lifetimes. Every reference in Rust has a lifetime, which is the scope for
    // which that reference is valid. Most of the time, lifetimes are implicit and inferred, just like most of the time,
    // types are inferred. We must annotate types when multiple types are possible. In a similar way, we must annotate
    // lifetimes when the lifetimes of references could be related in a few different ways.

    // The main aim of lifetimes is to prevent dangling references, which cause a program to reference data other than
    // the data it’s intended to reference.

    let r: &i32 = &1;
    {
        let x = 5;
        // error[E0597]: `x` does not live long enough
        // r = &x;
    }
    println!("r: {}", r);

    // Lifetime Annotation Syntax:
    // Lifetime annotations describe the relationships of the lifetimes of multiple references to each other without
    // affecting the lifetimes.

    // Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must start with an
    // apostrophe (') and are usually all lowercase and very short, like generic types. Most people use the name 'a.
    // We place lifetime parameter annotations after the & of a reference, using a space to separate the annotation from
    // the reference’s type.

    // let r1: &i32;        // a reference
    // let r2: &'a i32;     // a reference with an explicit lifetime
    // let rr: &'a mut i32; // a mutable reference with an explicit lifetime

    // TODO rest of chapter 10.3 lifetimes
}