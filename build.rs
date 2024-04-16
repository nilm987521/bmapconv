use anyhow::Result;
use vergen::EmitBuilder;

pub fn main() -> Result<()> {
    EmitBuilder::builder().git_sha(false).emit()?;
    Ok(())
}
