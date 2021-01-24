use rand::seq::SliceRandom;
use rand_xoshiro::{rand_core::SeedableRng, Xoshiro256StarStar};
use seq_io::fasta::{self, Reader, Record};
use std::{
    io::{BufWriter, Write},
    path::PathBuf,
};
use structopt::StructOpt;

const IS_DNA_ASCII: [bool; 256] = {
    let mut table = [false; 256];
    table[b'a' as usize] = true;
    table[b'A' as usize] = true;
    table[b'c' as usize] = true;
    table[b'C' as usize] = true;
    table[b'g' as usize] = true;
    table[b'G' as usize] = true;
    table[b't' as usize] = true;
    table[b'T' as usize] = true;
    table
};

const DNA_BASES: &[u8] = &[b'A', b'C', b'G', b'T'];

#[derive(StructOpt)]
struct Opt {
    /// FASTA file
    file: PathBuf,

    /// Maximum width of sequences
    #[structopt(short, long)]
    wrap: Option<usize>,

    /// Seed to initialize random number generator with
    #[structopt(short, long, default_value = "1234")]
    seed: u64,
}

fn main() -> anyhow::Result<()> {
    let opt = Opt::from_args();

    let mut reader = Reader::from_path(opt.file).unwrap();
    let out = std::io::stdout();
    let mut out = BufWriter::new(out.lock());

    let mut rng = Xoshiro256StarStar::seed_from_u64(opt.seed);

    while let Some(record) = reader.next() {
        let record = record?;
        fasta::write_head(&mut out, record.head())?;
        let seq: Vec<_> = record
            .seq()
            .iter()
            .filter_map(|x| {
                if x.is_ascii_alphabetic() {
                    Some(if IS_DNA_ASCII[*x as usize] {
                        *x
                    } else {
                        *DNA_BASES.choose(&mut rng).unwrap()
                    })
                } else {
                    None
                }
            })
            .collect();
        if let Some(wrap) = opt.wrap {
            fasta::write_wrap_seq(&mut out, &seq, wrap)?;
        } else {
            fasta::write_seq(&mut out, &seq)?;
        }
    }

    out.flush()?;
    Ok(())
}
