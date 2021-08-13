pub struct CommandParser {
  _oirinal_line: String,
  _where: usize,
  pub command: String,   // main command
  pub args: Vec<String>, // argument of the main shit
}

impl CommandParser {
  pub fn constructor(line: String) -> Self {
    Self {
      _oirinal_line: line,
      command: String::new(),
      args: Vec::new(),
      _where: 0,
    }
  }

  pub fn strings_parse(&mut self) {
    let cs: Vec<char> = self._oirinal_line.chars().collect();
    // let mut _cs: Vec<char>;
    let mut _isbeen_string: bool = false;

    self._oirinal_line = cs
      .iter()
      .map(|&n| {
        if n == '"' {
          _isbeen_string = !_isbeen_string;
        }

        if _isbeen_string {
          match n {
            ' ' => '_',
            _ => n,
          }
        } else {
          n
        }
      })
      .collect();
  }

  pub fn parse_it(&mut self) {
    self._oirinal_line.remove(self._oirinal_line.len() - 1);
    // remove space from original line
    self.strings_parse();

    let mut _commands: Vec<&str> = self._oirinal_line.split(' ').collect();
    self.command = _commands[self._where].to_string();
    self._where += 1;

    if let Some(pos) = _commands.iter().position(|x: &&str| *x == self.command) {
      _commands.remove(pos);
    }

    let mut args: Vec<String> = Vec::new();

    for d in _commands {
      args.push(d.to_string());
    }
    self.args = args;
  }
}

#[cfg(test)]
mod tests {

  use super::*;

  #[test]
  fn command_parser_no_arg() {
    // parse command into command object
    let mut commands = CommandParser::constructor("ls\n".to_owned());
    commands.parse_it();
    assert_eq!(commands.command, "ls".to_owned());
    assert_eq!(commands.args.len(), 0);
  }

  #[test]
  fn command_parser_one_arg() {
    // parse command into command object
    let mut commands = CommandParser::constructor("ls -al\n".to_owned());
    commands.parse_it();
    assert_eq!(commands.command, "ls".to_owned());
    assert_eq!(commands.args[0], "-al".to_owned());
    assert_eq!(commands.args.len(), 1);
  }
}
