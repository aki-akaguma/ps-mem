/*!
The process memory size listing.

`ps-mem` command is listing all process memory size.

# Feature

- minimum support rustc 1.65.0 (897e37553 2022-11-02)

# Command help

```text
ps-mem --help
```

```text
Usage:
  ps-mem [options]
  ps-mem [options] <command> {<arg1> <arg2> ...}

print processes memory by sort,
or print one processe memory

Options:
  -a, --all             all pid (include kernel threads)
  --sort <order>        sort by <order>: rss|swap|total
  --pid <number>        output only selected pid
  --sleep <number>      sleep <number> milli second
  -l, --cmdline         view command line

  -H, --help        display this help and exit
  -V, --version     display version information and exit
  -X <x-options>    x options. try -X help

Examples:
  Show all prosesses memory:
    ps-mem --all
  Show one prosess memory:
    ps-mem --pid 1234
  Invoke and show one prosess memory:
    ps-mem -- find / -type f
```

# Quick install

1. you can install this into cargo bin path:

```text
cargo install ps-mem
```

2. you can build debian package:

```text
cargo deb
```

and install **.deb** into your local repository of debian package.


# Examples

## Example 1: simple

```text
sudo ps-mem
```

## Example 2: the largest finder

you can see a largest memory process.

```text
sudo ps-mem --sort=total | tail
```

or

```text
sudo ps-mem --sort=total | aki-unbody -t 10
```

## Example 3: show one process memory

You see the process memory size of that pid is 1234.

```text
ps-mem --pid 1234
```

## Example 4: show invoked one prosess memory

You see the process memory size of the invoked find command.

```text
ps-mem find / -type f
```

*/

#[macro_use]
extern crate anyhow;

pub mod conf;
mod run;
mod util;

use flood_tide::HelpVersion;
use runnel::RunnelIoe;

const TRY_HELP_MSG: &str = "Try --help for help.";

/// execute ps-mem
///
/// params:
///   - sioe: stream in/out/err
///   - program: program name. etc. "ps-mem"
///   - args: parameter arguments.
///
/// return:
///   - ok: ()
///   - err: anyhow
///
pub fn execute<I, S>(sioe: &RunnelIoe, prog_name: &str, args: I) -> anyhow::Result<()>
where
    I: IntoIterator<Item = S>,
    S: AsRef<std::ffi::OsStr>,
{
    let args: Vec<String> = args
        .into_iter()
        .map(|s| s.as_ref().to_string_lossy().into_owned())
        .collect();
    let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
    match conf::parse_cmdopts(prog_name, &args_str) {
        Ok(conf) => run::run(sioe, &conf),
        Err(errs) => {
            if let Some(err) = errs.iter().find(|e| e.is_help() || e.is_version()) {
                sioe.pg_out().write_line(err.to_string())?;
                Ok(())
            } else {
                Err(anyhow!("{errs}\n{TRY_HELP_MSG}"))
            }
        }
    }
}
