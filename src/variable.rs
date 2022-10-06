use crate::sequence::random_sequence;

#[derive(Debug, Clone)]
pub struct Variable {
    sequence: String,
}
impl Variable {
    pub fn new(size: usize) -> Self {
        let sequence = random_sequence(size);
        Self { sequence }
    }
    pub fn new_set(size: usize, num: usize) -> Vec<Self> {
        (0..num).map(|_| Self::new(size)).collect()
    }
    pub fn sequence(&self) -> &str {
        &self.sequence
    }
}
