use super::*;
use crate::config::Verbosity;
use std::io::Write;

#[derive(Debug)]
pub(crate) struct StdoutEmitter
{
  verbosity: Verbosity,
}

impl StdoutEmitter
{
  pub(crate) fn new(verbosity: Verbosity) -> Self
  {
    Self { verbosity }
  }
}

impl Emitter for StdoutEmitter
{
  fn emit_formatted_file(
    &mut self,
    output: &mut dyn Write,
    FormattedFile {
      filename,
      formatted_text,
      ..
    }: FormattedFile<'_>,
  ) -> Result<EmitterResult, io::Error>
  {
    if self.verbosity != Verbosity::Quiet
    {
      writeln!(output, "{}:\n", filename)?;
    }
    write!(output, "{}", formatted_text)?;
    Ok(EmitterResult::default())
  }
}
