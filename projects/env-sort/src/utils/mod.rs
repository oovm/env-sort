

#[derive(Default, Debug)]
pub struct NonEmpty {
    pub vec: Vec<String>
}

impl NonEmpty {
    pub fn push(&mut self, new: String) {
        let trim = new.trim();
        if !trim.is_empty() {
            self.vec.push(trim.to_string())
        }
    }
}