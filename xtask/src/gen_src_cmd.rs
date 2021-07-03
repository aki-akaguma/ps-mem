use flood_tide_gen::{FixupType, MetaType, Pasc};

pub fn do_gen_src() -> anyhow::Result<()> {
    flood_tide_gen::do_gen_src(
        Pasc::Void,
        "xtask/src/ps-mem-cmd.txt",
        Some("src/conf/cmd.help.rs.txt"),
        Some("src/conf/cmd.match.rs.txt"),
        |opt_str| {
            let tup = match opt_str.lon_or_sho() {
                "pid" => (false, false, MetaType::I32),
                "sleep" => (false, false, MetaType::U32),
                "sort" => (false, false, MetaType::Other("opt_sort_order".into())),
                //
                "X" => (false, true, MetaType::Other("opt_uc_x_param".into())),
                _ => return None,
            };
            Some(FixupType::from_tuple(tup))
        },
    )
}
