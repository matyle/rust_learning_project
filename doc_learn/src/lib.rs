#![allow(unused)]
fn main() {
    /// `add_one` 将指定值加1
    ///
    /// # Examples
    ///
    /// ```
    /// let arg = 5;
    /// let answer = my_crate::add_one(arg);
    ///
    /// assert_eq!(6, answer);
    /// ```
    pub fn add_one(x: i32) -> Option<i32> {
        Some(x + 1)
    }
}
