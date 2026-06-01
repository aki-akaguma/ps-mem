//{{{ OptUcXParam
#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum OptUcXParam {
    #[default]
    Void,
    Help,
    RustVersionInfo,
    BaseDir(String),
}

impl ::std::str::FromStr for OptUcXParam {
    type Err = OptUcXParamParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let oc = match s {
            "void" => OptUcXParam::Void,
            "help" => OptUcXParam::Help,
            "rust-version-info" => OptUcXParam::RustVersionInfo,
            _ => {
                let bs = "base_dir=";
                if let Some(stripped) = s.strip_prefix(bs) {
                    OptUcXParam::BaseDir(stripped.to_string())
                } else {
                    let s = format!("can not parse '{s}'");
                    return Err(OptUcXParamParseError::new(s));
                }
            }
        };
        Ok(oc)
    }
}

impl ::std::fmt::Display for OptUcXParam {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let s = match *self {
            OptUcXParam::Void => "void",
            OptUcXParam::Help => "help",
            OptUcXParam::RustVersionInfo => "rust-version-info",
            OptUcXParam::BaseDir(_) => "base_dir=",
        };
        write!(f, "{s}")
    }
}
//}}} OptUcXParam

//{{{ OptUcXParamParseError
#[derive(Debug)]
pub struct OptUcXParamParseError {
    desc: String,
}

impl OptUcXParamParseError {
    fn new(s: String) -> OptUcXParamParseError {
        OptUcXParamParseError { desc: s }
    }
}

impl ::std::fmt::Display for OptUcXParamParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.desc.fmt(f)
    }
}

impl ::std::error::Error for OptUcXParamParseError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}
//}}} OptUcXParamParseError

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_display_void() {
        let col = OptUcXParam::Void;
        assert_eq!(format!("{col}"), "void");
    }
    #[test]
    fn test_display_help() {
        let col = OptUcXParam::Help;
        assert_eq!(format!("{col}"), "help");
    }
    #[test]
    fn test_display_rust_version_info() {
        let col = OptUcXParam::RustVersionInfo;
        assert_eq!(format!("{col}"), "rust-version-info");
    }
    #[test]
    fn test_display_base_dir() {
        let col = OptUcXParam::BaseDir("/tmp".to_string());
        assert_eq!(format!("{col}"), "base_dir=");
    }
    #[test]
    fn test_from_str_void() {
        let col: OptUcXParam = FromStr::from_str("void").unwrap();
        assert_eq!(col, OptUcXParam::Void);
    }
    #[test]
    fn test_from_str_help() {
        let col: OptUcXParam = FromStr::from_str("help").unwrap();
        assert_eq!(col, OptUcXParam::Help);
    }
    #[test]
    fn test_from_str_rust_version_info() {
        let col: OptUcXParam = FromStr::from_str("rust-version-info").unwrap();
        assert_eq!(col, OptUcXParam::RustVersionInfo);
    }
    #[test]
    fn test_from_str_base_dir() {
        let col: OptUcXParam = FromStr::from_str("base_dir=/tmp").unwrap();
        assert_eq!(col, OptUcXParam::BaseDir("/tmp".to_string()));
    }
    #[test]
    fn test_from_str_invalid() {
        let res: Result<OptUcXParam, _> = FromStr::from_str("invalid");
        assert!(res.is_err());
        assert_eq!(res.unwrap_err().to_string(), "can not parse 'invalid'");
    }
}
