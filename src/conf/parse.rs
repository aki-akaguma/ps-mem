//
use flood_tide::parse_simple_gnu_style;
use flood_tide::HelpVersion;
use flood_tide::{Arg, NameVal, Opt, OptNum};
use flood_tide::{OptParseError, OptParseErrors};

use crate::util::OptSortOrder;
use crate::util::OptUcXParam;
use std::str::FromStr;

//----------------------------------------------------------------------
include!("cmd.help.rs.txt");

//{{{ TEXT
const DESCRIPTIONS_TEXT: &str = r#"print processes memory by sort,
or print one processe memory
"#;
//const PARAMS_TEXT: &str = r#""#;
//const ARGUMENTS_TEXT: &str = r#""#;
//const ENV_TEXT: &str = r#""#;
const EXAMPLES_TEXT: &str = r#"Examples:
  Show all prosesses memory:
    ps-mem --all
  Show one prosess memory:
    ps-mem --pid 1234
  Show invoked one prosess memory:
    ps-mem find / -type f
"#;
//}}} TEXT

//----------------------------------------------------------------------
#[rustfmt::skip]
fn version_message(_program: &str) -> String {
    format!( "{} {}",
        env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"))
}

#[rustfmt::skip]
fn usage_message(program: &str) -> String {
    let usage1 = format!("  {} {}", program, "[options]");
    let usage2 = format!(
        "  {} {}",
        program, "[options] <command> {<arg1> <arg2> ...}"
    );
    format!("Usage:\n{}\n{}\n", usage1, usage2)
}

#[rustfmt::skip]
fn help_message(program: &str) -> String {
    let ver = version_message(program);
    let usa = usage_message(env!("CARGO_PKG_NAME"));
    [ &ver, "", &usa, DESCRIPTIONS_TEXT, OPTIONS_TEXT, EXAMPLES_TEXT].join("\n")
}

#[rustfmt::skip]
fn opt_uc_x_help_message(_program: &str) -> String {
    let z_opts = concat!(
        "Options:\n",
        "  -X rust-version-info     display package version info and exit\n",
        "  -X base_dir=<path>       set <path> is base directory\n",
    );
    z_opts.to_string()
}

#[rustfmt::skip]
fn opt_uc_x_package_version_info(_program: &str) -> String {
    #[cfg(feature = "debian_build")]
    {
        use std::io::Read;
        let mut string = String::new();
        let fnm = format!("/usr/share/doc/{}/rust-version-info.txt", env!("CARGO_PKG_NAME"));
        let file = std::fs::File::open(&fnm);
        match file {
            Ok(mut f) => {
                f.read_to_string(&mut string).unwrap();
                string
            },
            Err(err) => {
                format!("ERROR: {}: '{}'", err, fnm)
            },
        }
    }
    #[cfg(not(feature = "debian_build"))]
    {
        const VS: &str = include_str!(concat!(env!("OUT_DIR"), "/rust-version-info.txt"));
        VS.to_string()
    }
}

//----------------------------------------------------------------------
fn parse_match(conf: &mut CmdOptConf, nv: &NameVal<'_>) -> Result<(), OptParseError> {
    include!("cmd.match.rs.txt");
    Ok(())
}

pub fn parse_cmdopts(a_prog_name: &str, args: &[&str]) -> Result<CmdOptConf, OptParseErrors> {
    //
    let mut conf = CmdOptConf {
        prog_name: a_prog_name.to_string(),
        ..Default::default()
    };
    let (opt_free, r_errs) =
        parse_simple_gnu_style(&mut conf, &OPT_ARY, &OPT_ARY_SHO_IDX, args, parse_match);
    //
    if conf.is_help() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::help_message(&help_message(&conf.prog_name)));
        return Err(errs);
    }
    if conf.is_version() {
        let mut errs = OptParseErrors::new();
        errs.push(OptParseError::version_message(&version_message(
            &conf.prog_name,
        )));
        return Err(errs);
    }
    if !conf.opt_uc_x.is_empty() {
        if conf.is_opt_uc_x_help() {
            let mut errs = OptParseErrors::new();
            errs.push(OptParseError::help_message(&opt_uc_x_help_message(
                &conf.prog_name,
            )));
            return Err(errs);
        }
        if conf.is_opt_uc_x_package_version_info() {
            let mut errs = OptParseErrors::new();
            errs.push(OptParseError::help_message(&opt_uc_x_package_version_info(
                &conf.prog_name,
            )));
            return Err(errs);
        }
    }
    //
    {
        let errs = if let Err(errs) = r_errs {
            errs
        } else {
            OptParseErrors::new()
        };
        //
        if let Some(free) = opt_free {
            if !free.is_empty() {
                conf.arg_params.extend(free.iter().map(|s| s.to_string()));
            }
        };
        if !errs.is_empty() {
            return Err(errs);
        }
    }
    //
    Ok(conf)
}
