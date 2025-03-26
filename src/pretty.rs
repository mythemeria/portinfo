pub enum Verbosity {
  Normal,
  Verbose,
}

pub trait PrettyPrintable {
  fn print(&self, verbosity: Verbosity) {
    match verbosity {
      Verbosity::Normal  => self._print(),
      Verbosity::Verbose => self._print_v(),
    }
  }

  fn _print(&self);
  fn _print_v(&self);
}