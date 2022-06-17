# ps-mem

The process memory size listing.

`ps-mem` command is listing all process memory size.

## Feature

- minimum support rustc 1.56.1 (59eed8a2a 2021-11-01)

## Command help

```
ps-mem --help
```

```
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

## Quick install

1. you can install this into cargo bin path:

```
cargo install ps-mem
```

2. you can build debian package:

```
cargo deb
```

and install **.deb** into your local repository of debian package.


## Examples

### Example 1: simple

```
sudo ps-mem
```

### Example 2: the largest finder

you can see a largest memory process.

```
sudo ps-mem --sort=total | tail
```

or

```
sudo ps-mem --sort=total | aki-unbody -t 10
```

### Example 3: show one process memory

You see the process memory size of that pid is 1234.

```
ps-mem --pid 1234
```

### Example 4: show invoked one prosess memory

You see the process memory size of the invoked find command.

```
ps-mem find / -type f
```


# Changelogs

[This crate's changelog here.](https://github.com/aki-akaguma/ps-mem/blob/main/CHANGELOG.md)

# License

This project is licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
   https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   https://opensource.org/licenses/MIT)

at your option.
