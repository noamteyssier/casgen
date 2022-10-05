use rand::distributions::{Distribution, Uniform};

pub fn random_sequence(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let dist = Uniform::new(0, 4);
    dist.sample_iter(&mut rng)
        .take(length)
        .map(|x| match x {
            0 => 'A',
            1 => 'C',
            2 => 'G',
            3 => 'T',
            _ => panic!("Unexpected generated integer")
        })
        .fold(String::with_capacity(length), |mut s, x| {
            s.push(x);
            s
        })
}

pub fn reverse_complement(sequence: &str) -> String {
    sequence
        .chars()
        .rev()
        .map(|x| match x {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => panic!("Unexpected base pair found")
        })
        .collect()
}

pub fn generate_qual(sequence: &str) -> String {
    (0..sequence.len())
        .map(|_| '-')
        .collect()
}

pub fn fastq_rep(sequence: &str, construct_id: usize, index: usize) -> String {
    format!(
        "@seq_{}_construct_{}\n{}\n+\n{}\n", 
        index, 
        construct_id,
        sequence,
        generate_qual(sequence)
        )
}
