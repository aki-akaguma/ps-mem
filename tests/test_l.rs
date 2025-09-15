#[macro_use]
mod helper;

#[macro_use]
mod helper_l;

mod test_0_l {
    use libps_mem::*;
    use runnel::medium::stringio::{StringErr, StringIn, StringOut};
    use runnel::RunnelIoe;
    //
    #[test]
    fn test_help() {
        let (r, sioe) = do_execute!(["-H"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_help_long() {
        let (r, sioe) = do_execute!(["--help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), help_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version() {
        let (r, sioe) = do_execute!(["-V"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_version_long() {
        let (r, sioe) = do_execute!(["--version"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), version_msg!());
        assert!(r.is_ok());
    }
    #[test]
    fn test_invalid_opt() {
        let (r, sioe) = do_execute!(["-z"]);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}

mod test_0_x_options_l {
    use libps_mem::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    //
    #[test]
    fn test_x_option() {
        let (r, sioe) = do_execute!(["-X"]);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Missing option argument: X\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
    //
    #[test]
    fn test_x_option_help() {
        let (r, sioe) = do_execute!(["-X", "help"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_eq!(buff!(sioe, sout), x_help_msg!());
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let (r, sioe) = do_execute!(["-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        assert!(buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let (r, sioe) = do_execute!(["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(buff!(sioe, serr), "");
        // The first one should be executed and the program should exit.
        assert!(buff!(sioe, sout).contains("Options:"));
        assert!(!buff!(sioe, sout).contains("rustc"));
        assert!(r.is_ok());
    }
    #[test]
    fn test_x_option_invalid() {
        let (r, sioe) = do_execute!(["-X", "red"]);
        assert_eq!(
            buff!(sioe, serr),
            concat!(
                program_name!(),
                ": ",
                "Invalid option argument: X: can not parse 'red'\n",
                try_help_msg!()
            )
        );
        assert_eq!(buff!(sioe, sout), "");
        assert!(r.is_err());
    }
}

mod test_1_l {
    use assert_text::assert_text_eq;
    use libps_mem::*;
    use runnel::medium::stringio::*;
    use runnel::*;
    //
    #[test]
    fn test_t1() {
        let (r, sioe) = do_execute!(["-X", concat!("base_dir=", fixtures_1!()), "--sort=total"]);
        assert_eq!(buff!(sioe, serr), "");
        assert_text_eq!(
            buff!(sioe, sout),
            concat!(result_1_msg!(head), result_1_msg!(process))
        );
        assert!(r.is_ok());
    }
    #[test]
    fn test_t2() {
        let (r, sioe) = do_execute!([
            "-X",
            concat!("base_dir=", fixtures_1!()),
            "--sort=total",
            "-a"
        ]);
        assert_eq!(buff!(sioe, serr), "");
        assert_text_eq!(
            buff!(sioe, sout),
            concat!(result_1_msg!(head), result_1_msg!(all))
        );
        assert!(r.is_ok());
    }
    #[test]
    fn test_t3() {
        let (r, sioe) = do_execute!([
            "-X",
            concat!("base_dir=", fixtures_1!()),
            "--sort=total",
            "--pid=1794"
        ]);
        assert_eq!(buff!(sioe, serr), "");
        assert_text_eq!(
            buff!(sioe, sout),
            concat!(result_1_msg!(head), result_1_msg!(1794))
        );
        assert!(r.is_ok());
    }
    #[test]
    fn test_t4() {
        let (r, sioe) = do_execute!([
            "-X",
            concat!("base_dir=", fixtures_1!()),
            "--sort=total",
            "--pid=1794",
            "--cmdline",
        ]);
        assert_eq!(buff!(sioe, serr), "");
        assert_text_eq!(
            buff!(sioe, sout),
            concat!(result_1_msg!(head, cmdline), result_1_msg!(1794, cmdline))
        );
        assert!(r.is_ok());
    }
}
