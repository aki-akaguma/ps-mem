use crate::conf::{CmdOptConf, ProcsRec};
use crate::util::err::BrokenPipeError;
use crate::util::OptSortOrder;
use anyhow::Context;
use runnel::RunnelIoe;
use std::io::{LineWriter, Write};

use num_format::Locale;
use num_format::ToFormattedString;
use signal_hook::consts::signal;
use std::sync::atomic;
use std::sync::Arc;

pub fn run(sioe: &RunnelIoe, conf: &CmdOptConf) -> anyhow::Result<()> {
    //println!("{:?}", conf);
    let r = if conf.arg_params.is_empty() {
        let procs_rec = do_proc_in(conf)?;
        let mut sout = LineWriter::new(sioe.pg_out().lock());
        do_proc_out_list(conf, &procs_rec, &mut sout)
    } else {
        // invoke exec one process
        let procs_rec = do_proc_invoke(conf)?;
        let mut serr = LineWriter::new(sioe.pg_err().lock());
        do_proc_out_one(conf, &procs_rec, &mut serr)
    };
    if r.is_broken_pipe() {
        return Ok(());
    }
    r
}

fn do_proc_in(conf: &CmdOptConf) -> anyhow::Result<Vec<ProcsRec>> {
    let base_s = conf.base_dir();
    let mut sys = linux_procfs::System::new(base_s);
    let pid_vec = if conf.opt_pid > 0 {
        vec![conf.opt_pid]
    } else {
        sys.get_pids().context("Failed to get PIDs")?
    };
    //
    let mut recs: Vec<ProcsRec> = pid_vec
        .into_iter()
        .filter_map(|pid| {
            let pid_status = sys.get_pidentry_status(pid).ok().flatten()?;
            let pid_cmdline = if conf.flg_cmdline {
                sys.get_pidentry_cmdline(pid).ok().flatten()?
            } else {
                sys.get_pidentry_comm(pid).ok().flatten()?
            };
            //
            if !conf.flg_all && pid_status.state != b'Z' && pid_status.vm_size == 0 {
                return None;
            }
            //
            Some(ProcsRec {
                num: pid,
                state: pid_status.state,
                swap: pid_status.vm_swap,
                rss: pid_status.vm_rss,
                total: pid_status.vm_swap + pid_status.vm_rss,
                command: pid_cmdline.cmdline,
            })
        })
        .collect();
    //
    let state_key = |rec: &ProcsRec| if rec.state == b'Z' { b'Z' } else { b'A' };
    match conf.opt_sort {
        OptSortOrder::Empty => recs.sort_by_key(|a| a.num),
        OptSortOrder::Swap => recs.sort_by(|a, b| {
            (a.swap, a.rss, state_key(a), a.num).cmp(&(b.swap, b.rss, state_key(b), b.num))
        }),
        OptSortOrder::Rss => recs.sort_by(|a, b| {
            (a.rss, a.swap, state_key(a), a.num).cmp(&(b.rss, b.swap, state_key(b), b.num))
        }),
        OptSortOrder::Total => recs.sort_by(|a, b| {
            (a.total, a.rss, a.swap, state_key(a), a.num).cmp(&(
                b.total,
                b.rss,
                b.swap,
                state_key(b),
                b.num,
            ))
        }),
    }
    //
    Ok(recs)
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
        .with_context(|| format!("{prog} command failed to start"))?;
    let pid: i32 = child.id().try_into().with_context(|| {
        format!(
            "OS returned PID {} which is out of range for this tool (max: {})",
            child.id(),
            i32::MAX
        )
    })?;
    //
    let cmdline = match sys.get_pidentry_comm(pid) {
        Ok(Some(a)) => a.cmdline,
        Ok(None) => String::new(),
        Err(err) => return Err(anyhow!("sys.get_pidentry_comm({pid}): {err}")),
    };
    let mut vm_rss = 0;
    let mut vm_swap = 0;
    //
    let sleep_msec = std::time::Duration::from_millis(conf.opt_sleep as u64);
    loop {
        //
        if let Ok(Some(pid_status)) = sys.get_pidentry_status(pid) {
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
                break;
            }
            Ok(None) => {
                // nothing todo
            }
            Err(err) => {
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
