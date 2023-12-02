
#[derive(Debug, Default)]
pub struct App {
    pub exit: bool,
    pub text: String,
    pub digit: i32
}

impl App {
    pub fn new() -> Self {
        Self::default()
    } 
}
