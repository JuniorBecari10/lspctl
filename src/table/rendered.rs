// pre-rendered text and its visible width (excluding no-width characters, such as control ones)
pub struct Rendered {
    pub text: String,
    pub width: usize,
}

impl Rendered {
    pub fn new(text: String, width: usize) -> Self {
        Self { text, width }
    }
}
