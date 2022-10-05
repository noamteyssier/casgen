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

    eprintln!("Params: {:#?}", cli);
    let r1_filepath = format!("{}_R1.fastq", cli.prefix);
    let r2_filepath = format!("{}_R2.fastq", cli.prefix);
    let results_filepath = format!("{}_counts.tsv", cli.prefix);
    let sgrna_filepath = format!("{}_sgrna.tsv", cli.prefix);

    eprintln!(">> Writing R1 to: {}", r1_filepath);
    eprintln!(">> Writing R2 to: {}", r2_filepath);
    eprintln!(">> Writing counts to: {}", results_filepath);
    eprintln!(">> Writing sgRNAs to: {}", sgrna_filepath);

    let mut f1_writer = File::create(&r1_filepath)?;
    let mut f2_writer = File::create(&r2_filepath)?;
    let mut results_writer = File::create(&results_filepath)?;
    let mut sgrna_writer = File::create(&sgrna_filepath)?;

    let left_constant = Constant::new(cli.left_constant);
    let right_constant = Constant::new(cli.right_constant);
    let spacers = Spacer::new_set(cli.length_spacers, cli.num_variables);
    let variables = Variable::new_set(cli.length_variable, cli.num_constructs * cli.num_variables);
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
            write!(results_writer, "\t{}", constructs[*cid].get_variable(idx).sequence())?;
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
            writeln!(sgrna_writer, "{}\t{}", v.sequence(), cid)?;
        }
    }

    Ok(())
}
