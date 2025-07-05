mod public_module {
    pub fn send_answer(answer: &str) {
        // Return the answer
        println!("[PUBLIC MODULE/send_answer] This is the file module and it will return the answer.");
        println!("Note: {}", answer);
    }
}

pub fn init() {
    // Use module inside the file.
    public_module::send_answer("[PUBLIC MODULE/init] Importing answer in init function and this is the answer.");
}