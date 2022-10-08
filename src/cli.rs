use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about)]
pub struct Cli {
    /// Number of reads to generate
    #[clap(short = 'n', long, default_value = "1000")]
    pub num_reads: usize,

    /// Number of expected constructs
    #[clap(short = 'N', long, default_value = "10")]
    pub num_constructs: usize,

    /// Length of reads on each of end
    #[clap(short = 'r', long, default_value = "250")]
    pub read_size: usize,

    /// Length of variable size stagger
    #[clap(short = 'S', long, default_value = "8")]
    pub stagger: usize,

    /// Number of variables
    #[clap(long, default_value = "6")]
    pub num_variables: usize,

    /// Length of left-hand constant adapter region
    #[clap(short = 'c', long, default_value = "23")]
    pub left_adapter: usize,

    /// Length of right-hand constant adapter region
    #[clap(short = 'C', long, default_value = "53")]
    pub right_adapter: usize,

    /// Length of spacer regions
    #[clap(short = 's', long, default_value = "19")]
    pub length_spacers: usize,

    /// Length of variable (sgRNA) regions
    #[clap(short = 'v', long, default_value = "23")]
    pub length_variable: usize,

    /// Output prefix
    #[clap(short, long, default_value = "casgen")]
    pub prefix: String,
}
