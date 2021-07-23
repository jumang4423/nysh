extern crate regex;

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
            ' ' => return '_',
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

    let mut _commands: Vec<&str> = self._oirinal_line.split(" ").collect();
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
