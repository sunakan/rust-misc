/// This function adds 2 numbers
///
/// # Example
///
/// ```
/// use second::add;
///
/// add(1, 2);
/// ```
pub fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
