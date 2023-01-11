//{{{ OptSortOrder
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptSortOrder {
    Empty = 0,
    Swap,
    Rss,
    Total,
}

impl Default for OptSortOrder {
    fn default() -> OptSortOrder {
        OptSortOrder::Empty
    }
}

impl ::std::str::FromStr for OptSortOrder {
    type Err = OptSortOrderParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let oc = match s {
            "empty" => OptSortOrder::Empty,
            "swap" => OptSortOrder::Swap,
            "rss" => OptSortOrder::Rss,
            "total" => OptSortOrder::Total,
            _ => {
                let s = format!("can not parse '{s}'");
                return Err(OptSortOrderParseError::new(s));
            }
        };
        Ok(oc)
    }
}

impl ::std::fmt::Display for OptSortOrder {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let s = match *self {
            OptSortOrder::Empty => "empty",
            OptSortOrder::Swap => "swap",
            OptSortOrder::Rss => "rss",
            OptSortOrder::Total => "total",
        };
        write!(f, "{s}")
    }
}
//}}} OptSortOrder

//{{{ OptSortOrderParseError
#[derive(Debug)]
pub struct OptSortOrderParseError {
    desc: String,
}

impl OptSortOrderParseError {
    fn new(s: String) -> OptSortOrderParseError {
        OptSortOrderParseError { desc: s }
    }
}

impl ::std::fmt::Display for OptSortOrderParseError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        self.desc.fmt(f)
    }
}

impl ::std::error::Error for OptSortOrderParseError {
    fn description(&self) -> &str {
        self.desc.as_str()
    }
}
//}}} OptSortOrderParseError

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn test_display_empty() {
        let col = OptSortOrder::Empty;
        assert_eq!(format!("{}", col), "empty");
    }
    #[test]
    fn test_display_swap() {
        let col = OptSortOrder::Swap;
        assert_eq!(format!("{}", col), "swap");
    }
    #[test]
    fn test_display_rss() {
        let col = OptSortOrder::Rss;
        assert_eq!(format!("{}", col), "rss");
    }
    #[test]
    fn test_display_total() {
        let col = OptSortOrder::Total;
        assert_eq!(format!("{}", col), "total");
    }
    #[test]
    fn test_from_str_empty() {
        let col: OptSortOrder = match FromStr::from_str("empty") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptSortOrder::Empty);
    }
    #[test]
    fn test_from_str_swap() {
        let col: OptSortOrder = match FromStr::from_str("swap") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptSortOrder::Swap);
    }
    #[test]
    fn test_from_str_rss() {
        let col: OptSortOrder = match FromStr::from_str("rss") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptSortOrder::Rss);
    }
    #[test]
    fn test_from_str_total() {
        let col: OptSortOrder = match FromStr::from_str("total") {
            Ok(c) => c,
            Err(_) => {
                unreachable!();
            }
        };
        assert_eq!(col, OptSortOrder::Total);
    }
    #[test]
    fn test_from_str_invalid() {
        let _col: OptSortOrder = match FromStr::from_str("other") {
            Ok(_c) => _c,
            Err(e) => {
                assert_eq!(e.to_string(), "can not parse \'other\'");
                return;
            }
        };
        unreachable!();
    }
}
