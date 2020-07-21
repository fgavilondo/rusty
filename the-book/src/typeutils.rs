// we can use the pub keyword to decide which modules, types, functions, and methods in our code should be public,
// and by default everything else is private.
pub fn type_of<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}