/*!
The process memory size listing.

`ps-mem` command is listing all process memory size.

# Feature

- minimum support rustc 1.47.0 (18bf6b4f0 2020-10-07)

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
use std::io::Write;

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
pub fn execute(sioe: &RunnelIoe, prog_name: &str, args: &[&str]) -> anyhow::Result<()> {
    let env = conf::EnvConf::new();
    execute_env(sioe, prog_name, args, &env)
}

pub fn execute_env(
    sioe: &RunnelIoe,
    prog_name: &str,
    args: &[&str],
    env: &conf::EnvConf,
) -> anyhow::Result<()> {
    let conf = match conf::parse_cmdopts(prog_name, args) {
        Ok(conf) => conf,
        Err(errs) => {
            for err in errs.iter().take(1) {
                if err.is_help() || err.is_version() {
                    let _r = sioe.pout().lock().write_fmt(format_args!("{}\n", err));
                    return Ok(());
                }
            }
            return Err(anyhow!("{}\n{}", errs, TRY_HELP_MSG));
        }
    };
    run::run(sioe, &conf, env)
}
