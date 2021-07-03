pub use self::parse::parse_cmdopts;
use crate::util::OptUcXParam;
pub use parse::CmdOptConf;

mod parse;

impl CmdOptConf {
    pub fn base_dir(&self) -> String {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::BaseDir(s) = o {
                return s.clone();
            }
        }
        String::new()
    }
    pub fn is_opt_uc_x_help(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::Help = o {
                return true;
            }
        }
        false
    }
    pub fn is_opt_uc_x_package_version_info(&self) -> bool {
        for o in self.opt_uc_x.iter() {
            if let OptUcXParam::RustVersionInfo = o {
                return true;
            }
        }
        false
    }
}

#[derive(Debug)]
pub struct EnvConf {}
impl EnvConf {
    pub fn new() -> Self {
        //
        Self {}
    }
}
impl std::default::Default for EnvConf {
    fn default() -> EnvConf {
        EnvConf::new()
    }
}

#[derive(Debug, Default)]
pub struct ProcsRec {
    pub num: i32,
    pub state: u8,
    pub swap: usize,
    pub rss: usize,
    pub total: usize,
    pub command: String,
}
