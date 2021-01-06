/// First line is a short summary describing function
///
/// The next lines present detailed documentation. Code blocks start with triple backquotes and
/// have implicit `fn main()` inside and `extern crate <cratename>`, Assume we'are testing
///
/// ```
/// let result = doccomments::add(2,3);
/// assert_eq!(result, 5);
/// ```
#[cfg(test)]
// #[macro_use]
// extern crate pretty_assertions;
use pretty_assertions::{assert_eq, assert_ne};

pub mod lib_test {
    pub fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Usually doc comments may include sections "Examples", "Panics" and "Failures"
    ///
    /// The next function divides two numbers.
    ///
    /// # Examples
    ///
    /// ```
    /// let result = doccomments::div(10,2);
    /// assert_eq!(result, 5);
    /// ```
    ///
    /// # Panics
    ///
    /// The function panics if the second argument is zero
    ///
    /// ```rust, should panic
    ///  // panics on division by zero
    /// doccomments::div(10,0)
    /// ```

    pub fn div(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("Divide by zero Error");
        }

        a / b
    }

    pub fn mul(a: i32, b: i32) -> i32 {
        a * b
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_mul() {
            assert_eq!(mul(2, 3), 6);
        }
    }
}
