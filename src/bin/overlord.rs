#![feature(macro_rules)]
#![feature(struct_variant)]

extern crate liboverlord;
extern crate getopts;
use std::os;
use std::io::stdio;
use liboverlord::config::{Config};
use liboverlord::error::{OverlordResult, OverlordError};
use liboverlord::suite::{Suite};
use getopts::{optopt, optflag, getopts, OptGroup, usage, Matches};

static CONFIG_PATH: &'static str = "overlord.toml";

// Macro which returns from the current function and exits the process with an
// error code of 1.
//
// ```
// # #![allow(unreachable_code)]
// fn main(soup: bool) {
//   // Soup nazi does not like you therefore you never can have soup.
//   if soup {
//     exit!("{} bad!", "No soup for you");
//     // exit! returns and sets the exit code to 1.
//     println!("Never reached");
//   }
// }
// ```
macro_rules! exit {
  ($($arg:tt)*) => ({
    let mut stderr = stdio::stderr();
    let output = format_args!(std::fmt::format, $($arg)*);
    match stderr.write_str(output.as_slice()) {
      Ok(_) => return os::set_exit_status(1),
      Err(e) => fail!("Failed writing during exit! macro. {}", e)
    }
  });
}

// Defined as a static mostly so indentation looks correct.
static SUBCMD: &'static str = r#"

Subcommands:
    suites: List all available suites.
    help: Show this help message.
    run: Determine which suite a file belongs to an run it.
"#;

struct CLI<'a> {
  program: String,
  args: Vec<String>,
  opts: Vec<OptGroup>,
  matches: Matches
}

impl<'a> CLI<'a> {
  pub fn opts() -> Vec<OptGroup> {
    vec!(
      optflag("h", "help", "Show help for top level options"),
      optopt("c", "config", "TOML configuration file", CONFIG_PATH),
      optopt("C", "cwd", "Current working directory", "<path>")
    )
  }

  pub fn new(args: Vec<String>) -> CLI {
    let opts = CLI::opts();
    CLI {
      program: args.get(0).to_string(),
      opts: opts.clone(),
      args: args.clone(),
      matches: getopts(args.tail(), opts.as_slice()).unwrap()
    }
  }

  pub fn usage(&self) -> String {
    let header = format!("{} [TODO]", self.program);
    let cmds = getopts::usage(header.as_slice(), self.opts.as_slice());
    return format!("{} {}", cmds, SUBCMD);
  }

  fn config_path(&self) -> OverlordResult<Path> {
    let cwd = match self.matches.opt_str("C") {
      Some(v) => Path::new(v),
      None => os::getcwd()
    };

    // Get the configuration path from the flags.
    // Ensure the configuration path was found.
    let config_opt = cwd.join(match self.matches.opt_str("c") {
      Some(v) => v,
      None => CONFIG_PATH.to_string(),
    });

    let config_path = Path::new(config_opt);
    if config_path.exists() {
      Ok(config_path)
    } else {
      Err(OverlordError::new(format!(
        "Configuration path does not exist: \"{}\" \n\n {}",
        config_path.display(), self.usage()
      )))
    }
  }

  fn load_config(&self) -> OverlordResult<Config> {
    let path = try!(self.config_path());
    Config::parse(path)
  }

  fn cmd_help(&self) -> OverlordResult<()> {
    println!("{}", self.usage());
    Ok(())
  }

  fn cmd_suites(&self, config: &Config) -> OverlordResult<()> {
    // TODO: Figure out how to generate machine readable output?
    println!("Available suites:")
    for (name, suite) in config.manifest.suites.iter() {
      println!("    {} - {}", name, suite.title);
    }
    println!("")
    Ok(())
  }

  pub fn run<'a>(&'a self) -> OverlordResult<()> {
    let config = try!(self.load_config());

    if self.matches.free.len() < 1 {
      return Err(OverlordError::new("No subcommand provided.".to_string()));
    }

    let subcommand = self.matches.free.get(0).as_slice();
    match subcommand {
      "run" => self.cmd_suites(),
      "suites" => self.cmd_suites(&config),
      "help" => self.cmd_help(),
      _ => Err(OverlordError::new("Unknown subcommand".to_string()))
    }
  }
}

fn main() {
  let cli = CLI::new(os::args());
  match cli.run() {
    // Every error must be an OverlordError here.
    Err(e) => exit!("{} \n\n {}", e.human_error(), cli.usage()),
    // Success we don't care about the output here.
    _ => return
  }
}
