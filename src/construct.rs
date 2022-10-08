use rand::{distributions::Uniform, prelude::Distribution, thread_rng};

use crate::{adapter::Adapter, sequence::random_sequence, spacer::Spacer, variable::Variable};

#[derive(Debug)]
pub struct Construct {
    left_constant: Adapter,
    spacers: Vec<Spacer>,
    variables: Vec<Variable>,
    right_constant: Adapter,
}
impl Construct {
    pub fn new(
        left_constant: &Adapter,
        right_constant: &Adapter,
        spacers: &[Spacer],
        variables: &[Variable],
    ) -> Self {
        Self {
            left_constant: left_constant.clone(),
            spacers: spacers.to_owned(),
            variables: variables.to_owned(),
            right_constant: right_constant.clone(),
        }
    }

    pub fn sequence(&self) -> String {
        let mut s = String::with_capacity(500);
        s.push_str(self.left_constant.sequence());
        for idx in 0..self.spacers.len() {
            s.push_str(self.spacers[idx].sequence());
            s.push_str(self.variables[idx].sequence());
        }
        s.push_str(self.right_constant.sequence());
        s
    }

    pub fn staggered_sequence(&self, stagger_size: usize) -> String {
        let mut rng = thread_rng();
        let stagger_dist = Uniform::new(0, stagger_size);
        let stagger_left = random_sequence(stagger_dist.sample(&mut rng));
        let stagger_right = random_sequence(stagger_dist.sample(&mut rng));
        let mut seq = stagger_left;
        seq.push_str(&self.sequence());
        seq.push_str(&stagger_right);
        seq
    }

    pub fn get_variable(&self, idx: usize) -> &Variable {
        &self.variables[idx]
    }
}
