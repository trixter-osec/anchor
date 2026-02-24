use trixter_osec_anchor_cli::Opts;
use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    trixter_osec_anchor_cli::entry(Opts::parse())
}
