use crate::sequence::random_sequence;

#[derive(Debug, Clone)]
pub struct Adapter {
    sequence: String,
}
impl Adapter {
    pub fn new(size: usize) -> Self {
        let sequence = random_sequence(size);
        Self { sequence }
    }
    pub fn sequence(&self) -> &str {
        &self.sequence
    }
}
