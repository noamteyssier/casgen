use clap::Parser;


#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {

    /// Number of reads to generate
    #[clap(short, long, default_value="1000")]
    num_reads: usize,

    /// Number of expected constructs
    #[clap(short, long, default_value="10")]
    num_constructs: usize,

    /// Length of reads on each of end
    #[clap(short, long, default_value="250")]
    read_size: usize,

    /// Length of variable size stagger
    #[clap(short, long, default_value="8")]
    stagger: usize,

    /// Length of left-hand constant adapter region
    #[clap(short, long, default_value="23")]
    left_constant: usize,

    /// Length of right-hand constant adapter region
    #[clap(short, long, default_value="53")]
    right_constant: usize,

    /// Random seed to use in generation
    #[clap(short, long, default_value="42")]
    seed: usize,
}

fn main() {
    println!("Hello, world!");
}
