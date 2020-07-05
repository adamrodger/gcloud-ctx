use anyhow::Result;
use clap::Clap;

fn main() -> Result<()> {
    let opts = gcloud_ctx::Opts::parse();
    gcloud_ctx::run(opts)?;
    Ok(())
}
