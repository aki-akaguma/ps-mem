pub use self::parse::parse_cmdopts;
use crate::util::OptUcXParam;
pub use parse::CmdOptConf;

mod parse;

impl CmdOptConf {
    pub fn base_dir(&self) -> String {
        self.opt_uc_x
            .iter()
            .find_map(|o| match o {
                OptUcXParam::BaseDir(s) => Some(s.clone()),
                _ => None,
            })
            .unwrap_or_else(|| String::from("/"))
    }
    pub fn is_opt_uc_x_help(&self) -> bool {
        self.opt_uc_x.iter().any(|o| matches!(o, OptUcXParam::Help))
    }
    pub fn is_opt_uc_x_package_version_info(&self) -> bool {
        self.opt_uc_x
            .iter()
            .any(|o| matches!(o, OptUcXParam::RustVersionInfo))
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
