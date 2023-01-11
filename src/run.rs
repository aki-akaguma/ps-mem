use crate::conf::{CmdOptConf, EnvConf, ProcsRec};
use crate::util::err::BrokenPipeError;
use crate::util::OptSortOrder;
use runnel::RunnelIoe;
use std::io::{LineWriter, Write};

use num_format::Locale;
use num_format::ToFormattedString;
use signal_hook::consts::signal;
use std::cmp::Ordering;
use std::sync::atomic;
use std::sync::Arc;

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf, _env: &EnvConf) -> anyhow::Result<()> {
    //println!("{:?}", conf);
    let r = if conf.arg_params.is_empty() {
        let procs_rec = do_proc_in(conf);
        let mut sout = LineWriter::new(sioe.pout());
        do_proc_out_list(conf, &procs_rec, &mut sout)
    } else {
        // invoke exec one process
        let procs_rec = do_proc_invoke(conf)?;
        let mut serr = LineWriter::new(sioe.perr());
        do_proc_out_one(conf, &procs_rec, &mut serr)
    };
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn do_proc_in(conf: &CmdOptConf) -> Vec<ProcsRec> {
    let mut recs = Vec::new();
    let base_s = conf.base_dir();
    let mut sys = linux_procfs::System::new(base_s);
    let pid_vec = if conf.opt_pid > 0 {
        vec![conf.opt_pid]
    } else {
        sys.get_pids()
    };
    for &pid in pid_vec.iter() {
        let pid_status = match sys.get_pidentry_status(pid) {
            Some(a) => a,
            None => continue,
        };
        let pid_cmdline = if conf.flg_cmdline {
            match sys.get_pidentry_cmdline(pid) {
                Some(a) => a,
                None => continue,
            }
        } else {
            match sys.get_pidentry_comm(pid) {
                Some(a) => a,
                None => continue,
            }
        };
        //
        if !conf.flg_all && pid_status.state != b'Z' && pid_status.vm_size == 0 {
            continue;
        }
        //
        recs.push(ProcsRec {
            num: pid,
            state: pid_status.state,
            swap: pid_status.vm_swap,
            rss: pid_status.vm_rss,
            total: pid_status.vm_swap + pid_status.vm_rss,
            command: pid_cmdline.cmdline,
        });
    }
    //
    match conf.opt_sort {
        OptSortOrder::Empty => recs.sort_by(|a, b| a.num.cmp(&b.num)),
        OptSortOrder::Swap => recs.sort_by(|a, b| {
            let o = a.swap.cmp(&b.swap);
            match o {
                Ordering::Equal => {
                    let o = a.rss.cmp(&b.rss);
                    match o {
                        Ordering::Equal => {
                            let a_st = if a.state == b'Z' { b'Z' } else { b'A' };
                            let b_st = if b.state == b'Z' { b'Z' } else { b'A' };
                            let o = a_st.cmp(&b_st);
                            match o {
                                Ordering::Equal => a.num.cmp(&b.num),
                                _ => o,
                            }
                        }
                        _ => o,
                    }
                }
                _ => o,
            }
        }),
        OptSortOrder::Rss => recs.sort_by(|a, b| {
            let o = a.rss.cmp(&b.rss);
            match o {
                Ordering::Equal => {
                    let o = a.swap.cmp(&b.swap);
                    match o {
                        Ordering::Equal => {
                            let a_st = if a.state == b'Z' { b'Z' } else { b'A' };
                            let b_st = if b.state == b'Z' { b'Z' } else { b'A' };
                            let o = a_st.cmp(&b_st);
                            match o {
                                Ordering::Equal => a.num.cmp(&b.num),
                                _ => o,
                            }
                        }
                        _ => o,
                    }
                }
                _ => o,
            }
        }),
        OptSortOrder::Total => recs.sort_by(|a, b| {
            let o = a.total.cmp(&b.total);
            match o {
                Ordering::Equal => {
                    let o = a.rss.cmp(&b.rss);
                    match o {
                        Ordering::Equal => {
                            let o = a.swap.cmp(&b.swap);
                            match o {
                                Ordering::Equal => {
                                    let a_st = if a.state == b'Z' { b'Z' } else { b'A' };
                                    let b_st = if b.state == b'Z' { b'Z' } else { b'A' };
                                    let o = a_st.cmp(&b_st);
                                    match o {
                                        Ordering::Equal => a.num.cmp(&b.num),
                                        _ => o,
                                    }
                                }
                                _ => o,
                            }
                        }
                        _ => o,
                    }
                }
                _ => o,
            }
        }),
    }
    //
    recs
}

fn do_proc_out_list(
    conf: &CmdOptConf,
    procs: &[ProcsRec],
    wrt: &mut dyn Write,
) -> anyhow::Result<()> {
    let comm = if conf.flg_cmdline { "COMMAND" } else { "COMM" };
    let mut sum_rss = 0;
    let mut sum_swap = 0;
    let mut sum_total = 0;
    writeln!(
        wrt,
        "{:>7} - {:>9}   {:>9}   {:>9}   - {:<}",
        "PID", "RSS", "SWAP", "TOTAL", comm,
    )?;
    for rec in procs {
        writeln!(
            wrt,
            "{:7} - {:>9}Ki {:>9}Ki {:>9}Ki - {}",
            rec.num,
            rec.rss.to_formatted_string(&Locale::en),
            rec.swap.to_formatted_string(&Locale::en),
            rec.total.to_formatted_string(&Locale::en),
            rec.command
        )?;
        if conf.opt_pid == 0 {
            sum_rss += rec.rss;
            sum_swap += rec.swap;
            sum_total += rec.total;
        }
    }
    if conf.opt_pid == 0 {
        writeln!(
            wrt,
            "{:>7} - {:>9}Ki {:>9}Ki {:>9}Ki -",
            "Sum",
            sum_rss.to_formatted_string(&Locale::en),
            sum_swap.to_formatted_string(&Locale::en),
            sum_total.to_formatted_string(&Locale::en)
        )?;
    }
    //
    Ok(())
}

fn do_proc_invoke(conf: &CmdOptConf) -> anyhow::Result<ProcsRec> {
    let base_s = conf.base_dir();
    let mut sys = linux_procfs::System::new(base_s);
    //
    let term = Arc::new(atomic::AtomicBool::new(false));
    let _ = signal_hook::flag::register(signal::SIGINT, Arc::clone(&term));
    let _ = signal_hook::flag::register(signal::SIGTERM, Arc::clone(&term));
    //
    let prog = &conf.arg_params[0];
    let args = &conf.arg_params[1..];
    let mut child = std::process::Command::new(prog)
        .args(args)
        .spawn()
        .unwrap_or_else(|_| panic!("{prog} command failed to start"));
    let pid = child.id() as i32;
    //
    let cmdline = match sys.get_pidentry_comm(pid) {
        Some(a) => a.cmdline,
        None => String::new(),
    };
    let mut vm_rss = 0;
    let mut vm_swap = 0;
    //
    let sleep_msec = std::time::Duration::from_millis(conf.opt_sleep as u64);
    loop {
        //
        if let Some(pid_status) = sys.get_pidentry_status(pid) {
            vm_swap = vm_swap.max(pid_status.vm_swap);
            vm_rss = vm_rss.max(pid_status.vm_rss);
        };
        // check signal interrupt
        if term.load(atomic::Ordering::Relaxed) {
            let _ = child.kill();
            let _ = child.wait();
            break;
        }
        //
        std::thread::sleep(sleep_msec);
        match child.try_wait() {
            Ok(Some(_exit_status)) => {
                //println!("{:?}", _exit_status);
                break;
            }
            Ok(None) => {
                // nothing todo
            }
            Err(err) => {
                //eprintln!("error attempting to wait: {}", err);
                return Err(From::from(err));
            }
        }
    }
    //
    Ok(ProcsRec {
        num: pid,
        state: b'Z',
        swap: vm_swap,
        rss: vm_rss,
        total: vm_swap + vm_rss,
        command: cmdline,
    })
}

fn do_proc_out_one(_conf: &CmdOptConf, rec: &ProcsRec, wrt: &mut dyn Write) -> anyhow::Result<()> {
    writeln!(
        wrt,
        "pid: {}, rss: {}ki, swap: {}ki, total: {}ki, comm: {}",
        rec.num,
        rec.rss.to_formatted_string(&Locale::en),
        rec.swap.to_formatted_string(&Locale::en),
        rec.total.to_formatted_string(&Locale::en),
        rec.command
    )?;
    Ok(())
}
