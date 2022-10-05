use anyhow::Result;
use std::{collections::HashMap, fs::File, io::Write};

mod cli;
mod construct;
mod constant;
mod spacer;
mod variable;
mod sequence;

use clap::Parser;
use cli::Cli;
use constant::Constant;
use construct::Construct;
use rand::{thread_rng, distributions::Uniform, prelude::Distribution};
use sequence::{reverse_complement, fastq_rep};
use spacer::Spacer;
use variable::Variable;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut f1_writer = File::create("reads_R1.fastq")?;
    let mut f2_writer = File::create("reads_R2.fastq")?;
    let mut results_writer = File::create("counts.tsv")?;

    let left_constant = Constant::new(cli.left_constant);
    let right_constant = Constant::new(cli.right_constant);
    let spacers = Spacer::new_set(cli.length_spacers, cli.num_variables);
    let variables = Variable::new_set(cli.length_spacers, cli.num_constructs * cli.num_variables);
    let constructs = (0..cli.num_constructs)
        .map(|idx| {
            Construct::new(
                &left_constant,
                &right_constant,
                &spacers,
                &variables[(idx * cli.num_variables)..((idx*cli.num_variables) + cli.num_variables)]
                )
        })
        .collect::<Vec<Construct>>();

    let mut rng = thread_rng();
    let construct_id = Uniform::new(0, cli.num_constructs);
    let mut table: HashMap<usize, usize> = HashMap::new();
    for idx in 0..cli.num_reads {
        let id = construct_id.sample(&mut rng);
        *table.entry(id).or_default() += 1;
        let sequence = constructs[id].staggered_sequence(cli.stagger);
        let r1 = &sequence[0..250];
        let r2 = &reverse_complement(&sequence)[0..250];
        let r1_fq = fastq_rep(r1, id, idx);
        let r2_fq = fastq_rep(r2, id, idx);

        write!(f1_writer, "{}", r1_fq)?;
        write!(f2_writer, "{}", r2_fq)?;
    }

    writeln!(results_writer, "{}\t{}", "CID", "count")?;
    for (cid, count) in table.iter() {
        writeln!(results_writer, "{}\t{}", cid, count)?;
    }

    Ok(())
}
