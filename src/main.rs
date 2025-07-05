/// This is the main file.
/// It contains the main function.

/// Import the other files here
mod another_file;
mod file_module;
mod mod_folder;

/// Import the structs here
use crate::mod_folder::structure::StructImplementation;

/// Main function
fn main() {
    let new_note = "Hello, world!";
    let mut x = 5;
    x += 1;

    println!("{}! the value of x is: {}", new_note, x);

    // Use another file math_utils module
    let result: i32 = another_file::math_utils::add(1, 2);
    println!("The result is: {}", result);

    file_module::init();

    let structure_implementation: &str = StructImplementation::return_struct_info();
    println!("[MAIN/return_struct_info] The structure implemetation returns: {}", structure_implementation);
}
