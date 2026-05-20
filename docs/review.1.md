# Code Review Report for `ps-mem`

## Overview
The `ps-mem` project is a Rust-based command-line tool for listing process memory sizes on Linux. It is well-structured, follows modern Rust conventions (Edition 2021), and uses appropriate abstractions for I/O and CLI parsing.

## Strengths
- **Modular Design**: The project is logically divided into `conf`, `run`, and `util` modules, making it easy to navigate.
- **I/O Abstraction**: The use of the `runnel` crate for I/O abstraction (`RunnelIoe`) is an excellent practice, allowing for easier testing and redirection of standard streams.
- **Error Handling**: Comprehensive use of `anyhow` for error management and a custom trait for handling broken pipe errors (`BrokenPipeError`) shows attention to detail in CLI behavior.
- **Mocking for Tests**: The presence of `fixtures/t1/proc` indicates a robust testing strategy that can run against a simulated `/proc` filesystem.

## Areas for Improvement

### 1. Code Duplication in Sorting Logic
In `src/run.rs`, the `do_proc_in` function contains three large match arms for `OptSortOrder::Swap`, `OptSortOrder::Rss`, and `OptSortOrder::Total`. These arms are very similar, differing only in the primary field used for comparison.

**Recommendation**:
Refactor this logic by extracting the comparison into a helper function or using a closure that returns the key for sorting.

### 2. Error Handling in Process Invocation
In `src/run.rs`, the `do_proc_invoke` function uses `unwrap_or_else` with a `panic!` if the child process fails to start:
```rust
let mut child = std::process::Command::new(prog)
    .args(args)
    .spawn()
    .unwrap_or_else(|_| panic!("{prog} command failed to start"));
```
**Recommendation**:
Since the function returns `anyhow::Result<ProcsRec>`, it should return an error instead of panicking. This allows the caller to handle the failure gracefully.

### 3. PID Type Conversion
The code uses `child.id() as i32` in `do_proc_invoke`. While PIDs on Linux are typically within the range of `i32`, `child.id()` returns a `u32`. 

**Recommendation**:
Ensure that the conversion is safe or consistently use the type expected by the `linux-procfs` crate.

### 4. Code Cleanliness and Comments
- Some comments are low-signal, such as `// nothing todo` or `// println!("{:?}", conf);`.
- The naming convention `sioe` (Standard I/O and Error) is consistent but may be slightly obscure for new contributors. Consider using a more descriptive name or documenting the acronym clearly in `lib.rs`.

### 5. Polling Efficiency
In `do_proc_invoke`, the memory usage is tracked by polling the process in a loop with a sleep duration. The default sleep is 10ms, which might be slightly aggressive for long-running processes.

**Recommendation**:
Consider if a slightly higher default or a dynamic polling strategy might be beneficial, although for a memory tracking tool, high frequency might be desired.

## Conclusion
The codebase is of high quality and reflects a deep understanding of Rust and Linux system programming. Addressing the points above would further improve the maintainability and robustness of the tool.

---
Review Date: 2026-05-20
Reviewer: Gemini CLI Agent
