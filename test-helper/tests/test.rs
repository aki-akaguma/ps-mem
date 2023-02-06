const TARGET_EXE_PATH: &str = "../target/debug/ps-mem";
const EXE_SLEEP_PATH: &str = "../target/debug/exe-sleep";

mod test_1 {
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &str = super::TARGET_EXE_PATH;
    const EXE_SLEEP_PATH: &str = super::EXE_SLEEP_PATH;
    //
    #[cfg(target_os = "linux")]
    #[test]
    fn test_exec() {
        use assert_text::assert_text_match;
        //
        let oup = exec_target(TARGET_EXE_PATH, [EXE_SLEEP_PATH]);
        //
        assert_eq!(oup.stdout, "Have a good sleep.\n");
        //
        let lines: Vec<_> = oup.stderr.lines().collect();
        // "pid: 817768, rss: 4ki, swap: 0ki, total: 4ki, comm: exe-sleep"
        //assert_eq!(lines[0], "");
        assert_text_match!(
            lines[0],
            r"^pid: \d+, rss: +[\d,]+ki, swap: +[\d,]+ki, total: +[\d,]+ki, comm: exe-sleep$"
        );
        assert!(oup.status.success());
    }
}
