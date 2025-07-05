/// This is the struct implementation.
pub struct StructImplementation;

impl StructImplementation {
    pub fn return_struct_info() -> &'static str {
        const STRUCT_INFO: &str = "[STRUCTURE/return_struct_info] Comes from init_struct.rs";
        STRUCT_INFO
    }
}