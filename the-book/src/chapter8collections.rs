// see https://doc.rust-lang.org/std/collections/index.html for all other collections:
// Sequences: Vec, VecDeque, LinkedList
// Maps: HashMap, BTreeMap
// Sets: HashSet, BTreeSet
// Misc: BinaryHeap

pub(crate) fn collections_vectors() {
    println!();
    println!("8. Common Collections - Vectors");
    println!();

    // see https://doc.rust-lang.org/std/collections/index.html

    // Unlike the built-in array and tuple types, the data these collections point to is stored on the heap, which
    // means the amount of data does not need to be known at compile time and can grow or shrink as the program runs.

    // Vectors can only store values of the same type, implemented using generics
    let v: Vec<i32> = Vec::new();
    println!("v = {:?}", v);
    // it’s more common to create a Vec<T> that has initial values, and Rust provides the vec! macro for convenience.
    let mut v = vec![1, 2, 3];
    v.push(5);
    v.push(6);
    v.push(7);
    println!("v = {:?}", v);

    let third: &i32 = &v[2];
    println!("The third element is {}", third);

    match v.get(2) {
        Some(idx2) => println!("The third element is {}", idx2),
        None => println!("There is no third element."),
    }

    // let does_not_exist = &v[100]; // panic: index out of bounds
    let does_not_exist = v.get(100);
    println!("does_not_exist is {:?}", does_not_exist);


    // https://doc.rust-lang.org/std/iter/#the-three-forms-of-iteration
    // There are three common methods which can create iterators from a collection:
    //
    // iter(), which iterates over &T.
    // iter_mut(), which iterates over &mut T.
    // into_iter(), which iterates over T.

    // Rust's for loop syntax is actually sugar for iterators.

    for i in &v { // same as for i in v.iter()
        println!("{}", i);
    }

    for i in v { // same as for i in v.into_iter()
        println!("{}", i);
    }

    // We can also iterate over mutable references to each element in a mutable vector in order to make changes
    // to all the elements.
    let mut v = vec![100, 32, 57];
    for i in &mut v {  // same as for i in v.iter_mut()
        *i += 50;
    }

    // Trick: Using an Enum to Store Multiple Types

    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    println!("row = {:?}", row);
}
