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
        Self {}
    }
}
impl std::default::Default for EnvConf {
    fn default() -> EnvConf {
        EnvConf::new()
    }
}

impl<IKV, K, V> From<IKV> for EnvConf
where
    IKV: IntoIterator<Item = (K, V)>,
    K: AsRef<std::ffi::OsStr>,
    V: AsRef<std::ffi::OsStr>,
{
    fn from(_ary: IKV) -> Self {
        Self::new()
        /*
        let mut r = Self::new();
        for a in ary {
            match a.0.as_ref().to_string_lossy().to_string().as_str() {
                "AKI_GSUB_COLOR_SEQ_ST" => {
                    r.color_seq_start = a.1.as_ref().to_string_lossy().to_string();
                }
                _ => (),
            }
        }
        r
        */
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
