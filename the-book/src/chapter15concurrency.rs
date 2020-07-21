use std::thread;

pub(crate) fn threads() {
    println!();
    println!("15. Threads");
    println!();

    // creating an anonymous thread
    let handle = thread::spawn(|| {
        let thread = thread::current();
        assert_eq!(thread.name(), None);
        println!("Hello from {:?}", thread::current().id());
    });
    handle.join().unwrap();

    // creating a named thread
    let handle = thread::Builder::new()
        .name("named thread".into())
        .spawn(|| {
            let thread = thread::current();
            assert_eq!(thread.name(), Some("named thread"));
            println!("Hello from {:?}, aka '{}'", thread::current().id(), thread.name().unwrap());
        })
        .unwrap();

    handle.join().unwrap();
}