use anyhow::Result;
use std::{collections::HashMap, fs::File, io::Write};

mod cli;
mod adapter;
mod construct;
mod sequence;
mod spacer;
mod variable;

use clap::Parser;
use cli::Cli;
use adapter::Adapter;
use construct::Construct;
use rand::{distributions::Uniform, prelude::Distribution, thread_rng};
use sequence::{fastq_rep, reverse_complement};
use spacer::Spacer;
use variable::Variable;

fn main() -> Result<()> {
    let cli = Cli::parse();

    eprintln!("Params: {:#?}", cli);
    let r1_filepath = format!("{}_R1.fastq", cli.prefix);
    let r2_filepath = format!("{}_R2.fastq", cli.prefix);
    let results_filepath = format!("{}_counts.tsv", cli.prefix);
    let spacer_filepath = format!("{}_spacers.tsv", cli.prefix);
    let constant_filepath = format!("{}_constants.tsv", cli.prefix);

    eprintln!(">> Writing R1 to: {}", r1_filepath);
    eprintln!(">> Writing R2 to: {}", r2_filepath);
    eprintln!(">> Writing Counts to: {}", results_filepath);
    eprintln!(">> Writing Spacers to: {}", spacer_filepath);
    eprintln!(">> Writing Constants to: {}", constant_filepath);

    let mut f1_writer = File::create(&r1_filepath)?;
    let mut f2_writer = File::create(&r2_filepath)?;
    let mut results_writer = File::create(&results_filepath)?;
    let mut sgrna_writer = File::create(&spacer_filepath)?;
    let mut dr_writer = File::create(&constant_filepath)?;

    let left_adapter = Adapter::new(cli.left_adapter);
    let right_adapter = Adapter::new(cli.right_adapter);
    let spacers = Spacer::new_set(cli.length_spacers, cli.num_variables);
    let variables = Variable::new_set(cli.length_variable, cli.num_constructs * cli.num_variables);
    let constructs = (0..cli.num_constructs)
        .map(|idx| {
            Construct::new(
                &left_adapter,
                &right_adapter,
                &spacers,
                &variables
                    [(idx * cli.num_variables)..((idx * cli.num_variables) + cli.num_variables)],
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
        let r1 = &sequence[..250];
        let r2 = &reverse_complement(&sequence)[..250];
        let r1_fq = fastq_rep(r1, id, idx);
        let r2_fq = fastq_rep(r2, id, idx);

        write!(f1_writer, "{}", r1_fq)?;
        write!(f2_writer, "{}", r2_fq)?;
    }

    /*
     * Write Construct Count Table
     */

    write!(results_writer, "{}\t{}", "CID", "count")?;
    for idx in 0..cli.num_variables {
        write!(results_writer, "\tv{}", idx)?;
    }
    writeln!(results_writer)?;

    for (cid, count) in table.iter() {
        write!(results_writer, "{}\t{}", cid, count)?;
        for idx in 0..cli.num_variables {
            write!(
                results_writer,
                "\t{}",
                constructs[*cid].get_variable(idx).sequence()
            )?;
        }
        writeln!(results_writer)?;
    }

    /*
     * Write Variable Table
     */
    for cid in 0..cli.num_constructs {
        let c = &constructs[cid];
        for vid in 0..cli.num_variables {
            let v = c.get_variable(vid);
            writeln!(sgrna_writer, "{}\t{}\t{}", v.sequence(), cid, vid)?;
        }
    }

    /*
     * Write Spacers table
     */
    for (idx, spacer) in spacers.iter().enumerate() {
        writeln!(dr_writer, "{}\t{}", spacer.sequence(), idx)?;
    }

    Ok(())
}
