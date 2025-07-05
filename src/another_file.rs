// Public module, can be used in other files.
pub mod math_utils {
    // Public functions
    //a: i32, b: i32 are the parameters, both of type 32-bit integer.
    // -> i32 means the function returns a 32-bit integer.

    pub fn add(a: i32, b: i32) -> i32 {
        let result = a + b;
        // other_module::test_module::send_answer(result.to_string());
        // As we declare integer, we must return an integer.
        result
    }
}
