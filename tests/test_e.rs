#![cfg(not(miri))]

const TARGET_EXE_PATH: &str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

#[macro_use]
mod helper;

#[macro_use]
mod helper_e;

mod test_0_e {
    use assert_text::assert_text_eq;
    use exec_target::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-H"]);
        assert_eq!(oup.stderr, "");
        assert_text_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--help"]);
        assert_eq!(oup.stderr, "");
        assert_text_eq!(oup.stdout, help_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, ["-V"]);
        assert_eq!(oup.stderr, "");
        assert_text_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, ["--version"]);
        assert_eq!(oup.stderr, "");
        assert_text_eq!(oup.stdout, version_msg!());
        assert!(oup.status.success());
    }
    #[test]
    fn test_invalid_opt() {
        let oup = exec_target(TARGET_EXE_PATH, ["-z"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Invalid option: z\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    /*
    #[test]
    fn test_non_option() {
        let oup = exec_target(TARGET_EXE_PATH, args_from(""));
        assert!(!oup.status.success());
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.stderr, concat!(
            "Missing option: e\n",
            "Unexpected argument: \n",
            try_help_msg!()));
    }
    */
}

mod test_0_x_options_e {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_x_option() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Missing option argument: X\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
    //
    #[test]
    fn test_x_option_help() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help"]);
        assert_eq!(oup.stderr, "");
        assert_eq!(oup.stdout, x_help_msg!());
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_option_rust_version_info() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        assert!(oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_multiple_x_options() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "help", "-X", "rust-version-info"]);
        assert_eq!(oup.stderr, "");
        // The first one should be executed and the program should exit.
        assert!(oup.stdout.contains("Options:"));
        assert!(!oup.stdout.contains("rustc"));
        assert!(oup.status.success());
    }
    //
    #[test]
    fn test_x_option_invalid() {
        let oup = exec_target(TARGET_EXE_PATH, ["-X", "red"]);
        assert_eq!(
            oup.stderr,
            concat!(
                program_name!(),
                ": ",
                "Invalid option argument: X: can not parse 'red'\n",
                try_help_msg!()
            )
        );
        assert_eq!(oup.stdout, "");
        assert!(!oup.status.success());
    }
}

mod test_1_e {
    use assert_text::assert_text_eq;
    //use exec_target::args_from;
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_t1() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["-X", concat!("base_dir=", fixtures_1!()), "--sort=total"],
        );
        assert_eq!(oup.stderr, "");
        assert_text_eq!(
            oup.stdout,
            concat!(result_1_msg!(head), result_1_msg!(process))
        );
        assert!(oup.status.success());
    }
    #[test]
    fn test_t2() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            [
                "-X",
                concat!("base_dir=", fixtures_1!()),
                "--sort=total",
                "-a",
            ],
        );
        assert_eq!(oup.stderr, "");
        assert_text_eq!(oup.stdout, concat!(result_1_msg!(head), result_1_msg!(all)));
        assert!(oup.status.success());
    }
    #[test]
    fn test_t3() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            ["-X", concat!("base_dir=", fixtures_1!()), "--pid=1794"],
        );
        assert_eq!(oup.stderr, "");
        assert_text_eq!(
            oup.stdout,
            concat!(result_1_msg!(head), result_1_msg!(1794))
        );
        assert!(oup.status.success());
    }
    #[test]
    fn test_t4() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            [
                "-X",
                concat!("base_dir=", fixtures_1!()),
                "--pid=1794",
                "--cmdline",
            ],
        );
        assert_eq!(oup.stderr, "");
        assert_text_eq!(
            oup.stdout,
            concat!(result_1_msg!(head, cmdline), result_1_msg!(1794, cmdline))
        );
        assert!(oup.status.success());
    }
    /*
    #[cfg(target_os = "linux")]
    #[test]
    fn test_exec() {
        use assert_text::assert_text_match;
        //
        let oup = exec_target(TARGET_EXE_PATH, ["/bin/true"]);
        assert_eq!(oup.stdout, "");
        let lines: Vec<_> = oup.stderr.lines().collect();
        // "pid: 817768, rss: 4ki, swap: 0ki, total: 4ki, comm: true"
        //assert_eq!(lines[0], "");
        assert_text_match!(
            lines[0],
            r"^pid: \d+, rss: +[\d,]+ki, swap: +[\d,]+ki, total: +[\d,]+ki, comm: true$"
        );
        assert!(oup.status.success());
    }
    */
}
