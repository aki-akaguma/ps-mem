const TARGET_EXE_PATH: &'static str = env!(concat!("CARGO_BIN_EXE_", env!("CARGO_PKG_NAME")));

macro_rules! help_msg {
    () => {
        concat!(
            version_msg!(),
            "\n",
            indoc::indoc!(
                r#"
            Usage:
              ps-mem [options]
              ps-mem [options] <command> {<arg1> <arg2> ...}

            print processes memory by sort,
            or print one processe memory

            Options:
              -a, --all             all pid (include kernel threads)
              --sort <order>        sort by <order>: rss|swap|total
              --pid <number>        output only selected pid
              --sleep <number>      sleep <number> milli second
              -l, --cmdline         view command line

              -H, --help        display this help and exit
              -V, --version     display version information and exit
              -X <x-options>    x options. try -X help

            Examples:
              Show all prosesses memory:
                ps-mem --all
              Show one prosess memory:
                ps-mem --pid 1234
              Show invoked one prosess memory:
                ps-mem -- find / -type f
            "#
            ),
            "\n",
        )
    };
}

macro_rules! try_help_msg {
    () => {
        "Try --help for help.\n"
    };
}

macro_rules! program_name {
    () => {
        "ps-mem"
    };
}

macro_rules! version_msg {
    () => {
        concat!(program_name!(), " ", env!("CARGO_PKG_VERSION"), "\n")
    };
}

macro_rules! fixtures_1 {
    () => {
        "fixtures/t1"
    };
}

macro_rules! result_1_msg {
    (head) => {
        r"    PID -       RSS        SWAP       TOTAL   - COMM
"
    };
    (head, cmdline) => {
        r"    PID -       RSS        SWAP       TOTAL   - COMMAND
"
    };
    (1794) => {
        r"   1794 -     6,200Ki       420Ki     6,620Ki - winbindd
"
    };
    (1794, cmdline) => {
        r"   1794 -     6,200Ki       420Ki     6,620Ki - /usr/sbin/winbindd --foreground --no-process-group
"
    };
    (all) => {
        concat!(result_1_msg!(kernel), result_1_msg!(process),)
    };
    (kernel) => {
        r"      2 -         0Ki         0Ki         0Ki - kthreadd
      3 -         0Ki         0Ki         0Ki - rcu_gp
      4 -         0Ki         0Ki         0Ki - rcu_par_gp
      6 -         0Ki         0Ki         0Ki - kworker/0:0H-kblockd
      8 -         0Ki         0Ki         0Ki - mm_percpu_wq
      9 -         0Ki         0Ki         0Ki - ksoftirqd/0
     10 -         0Ki         0Ki         0Ki - rcu_preempt
     11 -         0Ki         0Ki         0Ki - migration/0
     12 -         0Ki         0Ki         0Ki - idle_inject/0
     14 -         0Ki         0Ki         0Ki - cpuhp/0
     15 -         0Ki         0Ki         0Ki - cpuhp/1
     16 -         0Ki         0Ki         0Ki - idle_inject/1
     17 -         0Ki         0Ki         0Ki - migration/1
     18 -         0Ki         0Ki         0Ki - ksoftirqd/1
     20 -         0Ki         0Ki         0Ki - kworker/1:0H-kblockd
     21 -         0Ki         0Ki         0Ki - cpuhp/2
     22 -         0Ki         0Ki         0Ki - idle_inject/2
     23 -         0Ki         0Ki         0Ki - migration/2
     24 -         0Ki         0Ki         0Ki - ksoftirqd/2
     26 -         0Ki         0Ki         0Ki - kworker/2:0H-kblockd
     27 -         0Ki         0Ki         0Ki - cpuhp/3
     28 -         0Ki         0Ki         0Ki - idle_inject/3
     29 -         0Ki         0Ki         0Ki - migration/3
     30 -         0Ki         0Ki         0Ki - ksoftirqd/3
     32 -         0Ki         0Ki         0Ki - kworker/3:0H-kblockd
     33 -         0Ki         0Ki         0Ki - kdevtmpfs
     34 -         0Ki         0Ki         0Ki - netns
     35 -         0Ki         0Ki         0Ki - rcu_tasks_kthre
     36 -         0Ki         0Ki         0Ki - kauditd
     38 -         0Ki         0Ki         0Ki - khungtaskd
     39 -         0Ki         0Ki         0Ki - oom_reaper
     40 -         0Ki         0Ki         0Ki - writeback
     41 -         0Ki         0Ki         0Ki - kcompactd0
     42 -         0Ki         0Ki         0Ki - ksmd
     43 -         0Ki         0Ki         0Ki - khugepaged
     90 -         0Ki         0Ki         0Ki - kintegrityd
     91 -         0Ki         0Ki         0Ki - kblockd
     92 -         0Ki         0Ki         0Ki - blkcg_punt_bio
     93 -         0Ki         0Ki         0Ki - irq/9-acpi
     94 -         0Ki         0Ki         0Ki - tpm_dev_wq
     95 -         0Ki         0Ki         0Ki - ata_sff
     96 -         0Ki         0Ki         0Ki - md
     97 -         0Ki         0Ki         0Ki - edac-poller
     98 -         0Ki         0Ki         0Ki - devfreq_wq
     99 -         0Ki         0Ki         0Ki - watchdogd
    101 -         0Ki         0Ki         0Ki - kswapd0
    102 -         0Ki         0Ki         0Ki - ecryptfs-kthrea
    104 -         0Ki         0Ki         0Ki - kthrotld
    105 -         0Ki         0Ki         0Ki - irq/24-PCIe PME
    106 -         0Ki         0Ki         0Ki - irq/25-PCIe PME
    107 -         0Ki         0Ki         0Ki - irq/25-pciehp
    108 -         0Ki         0Ki         0Ki - irq/25-s-pciehp
    109 -         0Ki         0Ki         0Ki - irq/26-PCIe PME
    110 -         0Ki         0Ki         0Ki - irq/26-pciehp
    111 -         0Ki         0Ki         0Ki - irq/26-s-pciehp
    112 -         0Ki         0Ki         0Ki - acpi_thermal_pm
    113 -         0Ki         0Ki         0Ki - irq/19-ata_piix
    114 -         0Ki         0Ki         0Ki - scsi_eh_0
    115 -         0Ki         0Ki         0Ki - scsi_tmf_0
    116 -         0Ki         0Ki         0Ki - scsi_eh_1
    117 -         0Ki         0Ki         0Ki - scsi_tmf_1
    119 -         0Ki         0Ki         0Ki - irq/19-ata_piix
    120 -         0Ki         0Ki         0Ki - scsi_eh_2
    121 -         0Ki         0Ki         0Ki - scsi_tmf_2
    122 -         0Ki         0Ki         0Ki - scsi_eh_3
    123 -         0Ki         0Ki         0Ki - scsi_tmf_3
    124 -         0Ki         0Ki         0Ki - vfio-irqfd-clea
    127 -         0Ki         0Ki         0Ki - irq/18-ehci_hcd
    129 -         0Ki         0Ki         0Ki - irq/23-ehci_hcd
    130 -         0Ki         0Ki         0Ki - irq/16-uhci_hcd
    131 -         0Ki         0Ki         0Ki - irq/21-uhci_hcd
    132 -         0Ki         0Ki         0Ki - irq/23-uhci_hcd
    133 -         0Ki         0Ki         0Ki - irq/19-uhci_hcd
    134 -         0Ki         0Ki         0Ki - irq/18-uhci_hcd
    136 -         0Ki         0Ki         0Ki - irq/12-i8042
    137 -         0Ki         0Ki         0Ki - irq/1-i8042
    138 -         0Ki         0Ki         0Ki - irq/8-rtc0
    139 -         0Ki         0Ki         0Ki - ipv6_addrconf
    150 -         0Ki         0Ki         0Ki - kstrp
    153 -         0Ki         0Ki         0Ki - kworker/u9:0
    166 -         0Ki         0Ki         0Ki - charger_manager
    174 -         0Ki         0Ki         0Ki - kworker/0:1H-kblockd
    175 -         0Ki         0Ki         0Ki - kworker/2:1H-kblockd
    176 -         0Ki         0Ki         0Ki - kworker/1:1H-kblockd
    177 -         0Ki         0Ki         0Ki - kworker/3:1H-kblockd
    225 -         0Ki         0Ki         0Ki - irq/17-pata_mar
    226 -         0Ki         0Ki         0Ki - scsi_eh_4
    227 -         0Ki         0Ki         0Ki - scsi_eh_5
    228 -         0Ki         0Ki         0Ki - irq/6-floppy
    229 -         0Ki         0Ki         0Ki - scsi_tmf_4
    230 -         0Ki         0Ki         0Ki - scsi_tmf_5
    231 -         0Ki         0Ki         0Ki - scsi_eh_6
    232 -         0Ki         0Ki         0Ki - usb-storage
    233 -         0Ki         0Ki         0Ki - scsi_tmf_6
    235 -         0Ki         0Ki         0Ki - irq/18-i801_smb
    236 -         0Ki         0Ki         0Ki - uas
    242 -         0Ki         0Ki         0Ki - kdmflush
    245 -         0Ki         0Ki         0Ki - kdmflush
    252 -         0Ki         0Ki         0Ki - kdmflush
    285 -         0Ki         0Ki         0Ki - jbd2/dm-0-8
    286 -         0Ki         0Ki         0Ki - ext4-rsv-conver
    372 -         0Ki         0Ki         0Ki - irq/7-parport0
    380 -         0Ki         0Ki         0Ki - loop0
    382 -         0Ki         0Ki         0Ki - loop1
    383 -         0Ki         0Ki         0Ki - loop2
    387 -         0Ki         0Ki         0Ki - loop3
    388 -         0Ki         0Ki         0Ki - loop4
    390 -         0Ki         0Ki         0Ki - loop5
    392 -         0Ki         0Ki         0Ki - loop6
    394 -         0Ki         0Ki         0Ki - loop7
    398 -         0Ki         0Ki         0Ki - loop8
    399 -         0Ki         0Ki         0Ki - loop9
    551 -         0Ki         0Ki         0Ki - irq/27-snd_hda_
    632 -         0Ki         0Ki         0Ki - jbd2/dm-1-8
    634 -         0Ki         0Ki         0Ki - ext4-rsv-conver
    671 -         0Ki         0Ki         0Ki - jbd2/dm-2-8
    672 -         0Ki         0Ki         0Ki - ext4-rsv-conver
   1181 -         0Ki         0Ki         0Ki - jbd2/sdb5-8
   1182 -         0Ki         0Ki         0Ki - ext4-rsv-conver
   1275 -         0Ki         0Ki         0Ki - irq/16-enp4s0
   1281 -         0Ki         0Ki         0Ki - irq/21-enp4s2
   1577 -         0Ki         0Ki         0Ki - irq/28-nvidia
   2595 -         0Ki         0Ki         0Ki - dm_bufio_cache
 460666 -         0Ki         0Ki         0Ki - kworker/2:1-events
 460788 -         0Ki         0Ki         0Ki - kworker/0:1-events
 462843 -         0Ki         0Ki         0Ki - kworker/u8:3-events_freezable_power_
 464437 -         0Ki         0Ki         0Ki - kworker/1:1-events
 465015 -         0Ki         0Ki         0Ki - kworker/u8:0-events_power_efficient
 465860 -         0Ki         0Ki         0Ki - kworker/u8:4-events_freezable_power_
 467377 -         0Ki         0Ki         0Ki - kworker/0:0-events
 468299 -         0Ki         0Ki         0Ki - kworker/2:3-events
 470720 -         0Ki         0Ki         0Ki - kworker/1:2
 470923 -         0Ki         0Ki         0Ki - kworker/u8:2-events_power_efficient
 472400 -         0Ki         0Ki         0Ki - kworker/u8:6-events_freezable_power_
 472720 -         0Ki         0Ki         0Ki - kworker/u8:5-events_power_efficient
 472793 -         0Ki         0Ki         0Ki - kworker/3:1-events
 472857 -         0Ki         0Ki         0Ki - kworker/3:2-events
 472911 -         0Ki         0Ki         0Ki - kworker/u8:1-events_freezable_power_
 472930 -         0Ki         0Ki         0Ki - kworker/3:0-events
"
    };
    (process) => {
        r" 216176 -         0Ki         0Ki         0Ki - bash
 216266 -       124Ki         0Ki       124Ki - hmon-rs.sh
 216254 -       132Ki         0Ki       132Ki - mon-rt.sh
   1533 -       252Ki        88Ki       340Ki - none
   1367 -       352Ki         4Ki       356Ki - avahi-daemon
   1615 -        12Ki       352Ki       364Ki - dnsmasq
   1638 -        12Ki       352Ki       364Ki - dnsmasq
   1801 -       364Ki         8Ki       372Ki - dnsmasq
   1274 -       428Ki         0Ki       428Ki - acpid
 215626 -       228Ki       228Ki       456Ki - ssh-agent
 472937 -       608Ki         0Ki       608Ki - copy_proc.sh
   1310 -       656Ki         0Ki       656Ki - osspd
 472939 -     1,148Ki         0Ki     1,148Ki - copy_proc.sh
   1777 -     1,272Ki         0Ki     1,272Ki - agetty
   1702 -     1,328Ki         0Ki     1,328Ki - rtkit-daemon
 217474 -       984Ki       428Ki     1,412Ki - dbus-launch
 215649 -     1,304Ki       140Ki     1,444Ki - fcitx-dbus-watc
   1355 -     1,476Ki         0Ki     1,476Ki - wpa_supplicant
   1241 -     1,504Ki         8Ki     1,512Ki - systemd-timesyn
 216260 -     1,240Ki       308Ki     1,548Ki - ssh-agent
 217475 -     1,172Ki       448Ki     1,620Ki - dbus-daemon
   1304 -     1,620Ki         0Ki     1,620Ki - lxcfs
   1358 -     1,628Ki         0Ki     1,628Ki - atd
   1614 -     1,348Ki       324Ki     1,672Ki - dnsmasq
   1761 -     1,684Ki        16Ki     1,700Ki - kerneloops
   1637 -     1,420Ki       328Ki     1,748Ki - dnsmasq
   1766 -     1,772Ki         0Ki     1,772Ki - kerneloops
   1350 -     1,880Ki         0Ki     1,880Ki - systemd-machine
   1290 -     2,008Ki         4Ki     2,012Ki - cron
   1740 -     2,028Ki         0Ki     2,028Ki - inetd
 215861 -     1,544Ki       624Ki     2,168Ki - agent
 216010 -     2,236Ki         0Ki     2,236Ki - obexd
   2036 -     2,260Ki         0Ki     2,260Ki - qmgr
   1301 -     2,284Ki         0Ki     2,284Ki - irqbalance
   2034 -     2,260Ki        28Ki     2,288Ki - master
   1461 -     2,412Ki         0Ki     2,412Ki - sshd
   1446 -     2,448Ki         0Ki     2,448Ki - privoxy
 215673 -     2,432Ki        84Ki     2,516Ki - dbus-daemon
 216050 -     2,600Ki         0Ki     2,600Ki - gvfs-mtp-volume
   1285 -     2,652Ki         0Ki     2,652Ki - avahi-daemon
   2139 -     2,660Ki         0Ki     2,660Ki - upowerd
   1911 -     1,744Ki       932Ki     2,676Ki - smbd-notifyd
 215486 -     2,952Ki         0Ki     2,952Ki - gvfs-udisks2-vo
 215645 -     2,804Ki       152Ki     2,956Ki - dbus-daemon
   1411 -     3,016Ki         0Ki     3,016Ki - ModemManager
 217467 -     2,392Ki       656Ki     3,048Ki - ssh
 216256 -     3,052Ki         0Ki     3,052Ki - ssh
 216298 -     3,124Ki         0Ki     3,124Ki - ssh
 216596 -     3,324Ki         0Ki     3,324Ki - xdg-permission-
   1273 -     3,372Ki        16Ki     3,388Ki - accounts-daemon
   1347 -     3,428Ki         0Ki     3,428Ki - systemd-logind
   1913 -     3,020Ki       420Ki     3,440Ki - winbindd
   1466 -     2,224Ki     1,256Ki     3,480Ki - lightdm
 215962 -     2,420Ki     1,068Ki     3,488Ki - goa-identity-se
 215668 -     3,224Ki       352Ki     3,576Ki - at-spi-bus-laun
   1235 -     3,604Ki         8Ki     3,612Ki - systemd-network
 216026 -     3,612Ki         0Ki     3,612Ki - gvfs-goa-volume
 215807 -     3,156Ki       468Ki     3,624Ki - gvfsd-trash
 216208 -     1,788Ki     1,844Ki     3,632Ki - bash
 215675 -     3,360Ki       304Ki     3,664Ki - xfconfd
 224242 -     3,728Ki         0Ki     3,728Ki - gvfsd-dnssd
 216225 -     2,564Ki     1,176Ki     3,740Ki - bash
 216277 -     2,216Ki     1,536Ki     3,752Ki - bash
 216041 -     3,780Ki         0Ki     3,780Ki - gvfs-afc-volume
 237399 -     1,772Ki     2,036Ki     3,808Ki - bash
 230426 -     1,748Ki     2,064Ki     3,812Ki - bash
 216579 -     3,816Ki         0Ki     3,816Ki - xdg-document-po
 237409 -     1,784Ki     2,040Ki     3,824Ki - bash
 230438 -     1,796Ki     2,040Ki     3,836Ki - bash
   1336 -     3,816Ki        20Ki     3,836Ki - rsyslogd
 215469 -     3,560Ki       300Ki     3,860Ki - gvfsd
 225098 -     1,784Ki     2,080Ki     3,864Ki - bash
 216046 -     3,916Ki         0Ki     3,916Ki - gvfs-gphoto2-vo
 215678 -     3,924Ki         0Ki     3,924Ki - at-spi2-registr
 220701 -     2,064Ki     1,888Ki     3,952Ki - bash
 237417 -     1,788Ki     2,168Ki     3,956Ki - bash
 216267 -     3,948Ki        32Ki     3,980Ki - sudo
   1342 -     3,964Ki        24Ki     3,988Ki - smartd
 222992 -     1,704Ki     2,292Ki     3,996Ki - bash
   1293 -     3,928Ki       176Ki     4,104Ki - dbus-daemon
   1914 -     3,692Ki       420Ki     4,112Ki - winbindd
 215464 -     3,964Ki       348Ki     4,312Ki - dbus-daemon
 215484 -     4,108Ki       288Ki     4,396Ki - gvfsd-fuse
 215459 -     3,960Ki       480Ki     4,440Ki - gnome-keyring-d
 224227 -     4,556Ki         0Ki     4,556Ki - gvfsd-network
   1912 -     3,632Ki       928Ki     4,560Ki - cleanupd
 223680 -     2,772Ki     1,796Ki     4,568Ki - bash
   1755 -     4,056Ki       632Ki     4,688Ki - whoopsie
 216157 -     3,756Ki       956Ki     4,712Ki - bash
 215444 -       792Ki     4,016Ki     4,808Ki - (sd-pam)
 238663 -     4,884Ki         4Ki     4,888Ki - cupsd
 215410 -     3,128Ki     1,832Ki     4,960Ki - lightdm
 216272 -     5,036Ki         0Ki     5,036Ki - hmon-rs
 215983 -     4,884Ki       352Ki     5,236Ki - dconf-service
 470346 -     5,316Ki         0Ki     5,316Ki - pickup
 216139 -     3,228Ki     2,120Ki     5,348Ki - bash
   1873 -     4,936Ki       416Ki     5,352Ki - winbindd
 224409 -     4,112Ki     1,280Ki     5,392Ki - bash
 453694 -     5,424Ki         0Ki     5,424Ki - bash
 460819 -     5,680Ki         0Ki     5,680Ki - bash
   1312 -     5,436Ki       316Ki     5,752Ki - polkitd
 234542 -     5,072Ki       856Ki     5,928Ki - bash
    397 -     3,332Ki     2,628Ki     5,960Ki - systemd-udevd
   1747 -     5,928Ki        44Ki     5,972Ki - nmbd
 464373 -     5,980Ki         0Ki     5,980Ki - bash
 238664 -     5,968Ki       128Ki     6,096Ki - cups-browsed
   1915 -     5,256Ki       888Ki     6,144Ki - lpqd
 215870 -     6,336Ki         0Ki     6,336Ki - zeitgeist-daemo
 215438 -     6,236Ki       372Ki     6,608Ki - systemd
   1794 -     6,200Ki       420Ki     6,620Ki - winbindd
 464776 -     6,848Ki         0Ki     6,848Ki - ssh
 215995 -     7,232Ki         0Ki     7,232Ki - evolution-addre
   1272 -     4,968Ki     2,444Ki     7,412Ki - systemd-resolve
 215968 -     7,400Ki       456Ki     7,856Ki - evolution-calen
   1899 -     7,008Ki       892Ki     7,900Ki - smbd
 217443 -     8,212Ki       252Ki     8,464Ki - gvfsd-metadata
      1 -     7,276Ki     1,232Ki     8,508Ki - systemd
 215933 -     8,644Ki        36Ki     8,680Ki - evolution-sourc
 215783 -     3,416Ki     5,436Ki     8,852Ki - xfce4-power-man
 215785 -     4,412Ki     5,400Ki     9,812Ki - polkit-gnome-au
 215840 -     8,964Ki     1,460Ki    10,424Ki - zeitgeist-datah
 215883 -    10,452Ki        44Ki    10,496Ki - zeitgeist-fts
 215765 -     8,012Ki     2,772Ki    10,784Ki - xfce4-power-man
   1410 -     8,276Ki     2,716Ki    10,992Ki - colord
   1306 -     3,976Ki     7,104Ki    11,080Ki - networkd-dispat
   1460 -     4,944Ki     6,552Ki    11,496Ki - unattended-upgr
 215767 -     8,048Ki     3,864Ki    11,912Ki - cairo-clock
 215754 -     9,052Ki     3,172Ki    12,224Ki - panel-2-actions
 224140 -    10,500Ki     2,016Ki    12,516Ki - gvfsd-smb
 215741 -     9,400Ki     3,364Ki    12,764Ki - panel-6-systray
 217013 -     4,236Ki     8,796Ki    13,032Ki - snap
 463041 -    13,224Ki         0Ki    13,224Ki - chrome
 215452 -     7,328Ki     6,412Ki    13,740Ki - autojack
   1439 -     8,140Ki     5,648Ki    13,788Ki - libvirtd
 215717 -     7,832Ki     6,828Ki    14,660Ki - xfsettingsd
 215743 -    10,576Ki     4,532Ki    15,108Ki - panel-14-power-
 215740 -    13,064Ki     2,208Ki    15,272Ki - panel-25-cpugra
 215755 -    11,640Ki     4,080Ki    15,720Ki - panel-22-thunar
 215742 -    11,756Ki     4,112Ki    15,868Ki - panel-5-notific
 215859 -     8,368Ki     7,580Ki    15,948Ki - tracker-miner-f
 215782 -     6,744Ki     9,244Ki    15,988Ki - nm-applet
 215515 -    12,880Ki     3,656Ki    16,536Ki - xfce4-session
 463064 -    17,076Ki         0Ki    17,076Ki - chrome
 215770 -    13,324Ki     4,184Ki    17,508Ki - spectrum-sequel
   1344 -    13,012Ki     6,328Ki    19,340Ki - snapd
 215842 -    18,756Ki     1,288Ki    20,044Ki - pulseaudio
 215686 -    16,220Ki     4,828Ki    21,048Ki - xfce4-screensav
 215826 -     6,828Ki    15,648Ki    22,476Ki - applet.py
 233500 -    16,512Ki     6,048Ki    22,560Ki - RDD Process
 215949 -     3,760Ki    19,980Ki    23,740Ki - goa-daemon
 215863 -     5,548Ki    18,224Ki    23,772Ki - blueman-applet
 215660 -    19,560Ki     5,608Ki    25,168Ki - notify-osd
 215970 -     7,528Ki    18,276Ki    25,804Ki - blueman-tray
 215639 -    17,396Ki     8,704Ki    26,100Ki - fcitx
 215843 -     8,344Ki    17,800Ki    26,144Ki - kdeconnectd
 215812 -    20,240Ki     6,676Ki    26,916Ki - tracker-store
 215835 -     6,564Ki    22,100Ki    28,664Ki - evolution-alarm
    337 -    29,224Ki        12Ki    29,236Ki - systemd-journal
 215710 -    25,676Ki     3,788Ki    29,464Ki - xfce4-panel
 216188 -    23,640Ki     5,916Ki    29,556Ki - gnome-terminal-
 215665 -    25,532Ki     9,764Ki    35,296Ki - mozc_server
 215772 -    21,056Ki    18,708Ki    39,764Ki - pulseeffects
 463038 -    47,808Ki         0Ki    47,808Ki - chrome
 463039 -    48,164Ki         0Ki    48,164Ki - chrome
 470378 -    50,216Ki         0Ki    50,216Ki - chrome
 215700 -    39,800Ki    11,340Ki    51,140Ki - xfwm4
 216134 -    43,752Ki    11,724Ki    55,476Ki - xfce4-terminal
 215748 -    44,568Ki    12,032Ki    56,600Ki - panel-20-pulsea
 215703 -    52,388Ki     5,564Ki    57,952Ki - Thunar
 463121 -    70,652Ki         0Ki    70,652Ki - chrome
 463158 -    75,080Ki         0Ki    75,080Ki - chrome
 215752 -    44,116Ki    31,388Ki    75,504Ki - panel-21-weathe
 463185 -    77,080Ki         0Ki    77,080Ki - chrome
 463061 -    78,092Ki         0Ki    78,092Ki - chrome
 463174 -    78,796Ki         0Ki    78,796Ki - chrome
 463164 -    81,468Ki         0Ki    81,468Ki - chrome
 463119 -    86,076Ki         0Ki    86,076Ki - chrome
 464367 -    86,368Ki         0Ki    86,368Ki - geany
 217427 -    64,024Ki    22,732Ki    86,756Ki - virt-manager
 463129 -    90,724Ki         0Ki    90,724Ki - chrome
 463134 -    91,564Ki         0Ki    91,564Ki - chrome
 463317 -   115,356Ki         0Ki   115,356Ki - chrome
 463231 -   124,652Ki         0Ki   124,652Ki - chrome
 233278 -    91,652Ki    37,272Ki   128,924Ki - Privileged Cont
   1493 -   110,852Ki    20,460Ki   131,312Ki - Xorg
 463059 -   138,868Ki         0Ki   138,868Ki - chrome
 459104 -   147,840Ki         0Ki   147,840Ki - Privileged Cont
 215735 -   153,152Ki    15,832Ki   168,984Ki - xfdesktop
 233396 -   146,424Ki    44,576Ki   191,000Ki - Web Content
 463111 -   215,300Ki         0Ki   215,300Ki - chrome
 233425 -   247,384Ki    14,672Ki   262,056Ki - Web Content
 463305 -   266,604Ki         0Ki   266,604Ki - chrome
 459168 -   297,784Ki         0Ki   297,784Ki - WebExtensions
 459239 -   442,204Ki         0Ki   442,204Ki - Web Content
 233339 -   459,348Ki    36,728Ki   496,076Ki - WebExtensions
 459054 -   833,040Ki         0Ki   833,040Ki - MainThread
 462847 -   936,284Ki         0Ki   936,284Ki - chrome
 233231 - 1,176,272Ki    99,792Ki 1,276,064Ki - MainThread
    Sum - 8,039,024Ki   722,940Ki 8,761,964Ki -
"
    };
}

mod test_0 {
    use assert_text::assert_text_eq;
    use exec_target::exec_target;
    //use exec_target::args_from;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;
    //
    #[test]
    fn test_help() {
        let oup = exec_target(TARGET_EXE_PATH, &["-H"]);
        assert_eq!(oup.stderr, "");
        assert_text_eq!(oup.stdout, help_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_help_long() {
        let oup = exec_target(TARGET_EXE_PATH, &["--help"]);
        assert_eq!(oup.stderr, "");
        assert_text_eq!(oup.stdout, help_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_version() {
        let oup = exec_target(TARGET_EXE_PATH, &["-V"]);
        assert_eq!(oup.stderr, "");
        assert_text_eq!(oup.stdout, version_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_version_long() {
        let oup = exec_target(TARGET_EXE_PATH, &["--version"]);
        assert_eq!(oup.stderr, "");
        assert_text_eq!(oup.stdout, version_msg!());
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_invalid_opt() {
        let oup = exec_target(TARGET_EXE_PATH, &["-z"]);
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
        assert_eq!(oup.status.success(), false);
    }
    /*
    #[test]
    fn test_non_option() {
        let oup = exec_target(TARGET_EXE_PATH, args_from(""));
        assert_eq!(oup.status.success(), false);
        assert_eq!(oup.stdout, "");
        assert_eq!(oup.stderr, concat!(
            "Missing option: e\n",
            "Unexpected argument: \n",
            try_help_msg!()));
    }
    */
} // mod test_0

mod test_1 {
    use assert_text::assert_text_eq;
    use assert_text::assert_text_match;
    //use exec_target::args_from;
    use exec_target::exec_target;
    const TARGET_EXE_PATH: &'static str = super::TARGET_EXE_PATH;

    //
    #[test]
    fn test_t1() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            &["-X", concat!("base_dir=", fixtures_1!()), "--sort=total"],
        );
        assert_eq!(oup.stderr, "");
        assert_text_eq!(
            oup.stdout,
            concat!(result_1_msg!(head), result_1_msg!(process))
        );
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_t2() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            &[
                "-X",
                concat!("base_dir=", fixtures_1!()),
                "--sort=total",
                "-a",
            ],
        );
        assert_eq!(oup.stderr, "");
        assert_text_eq!(oup.stdout, concat!(result_1_msg!(head), result_1_msg!(all)));
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_t3() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            &["-X", concat!("base_dir=", fixtures_1!()), "--pid=1794"],
        );
        assert_eq!(oup.stderr, "");
        assert_text_eq!(
            oup.stdout,
            concat!(result_1_msg!(head), result_1_msg!(1794))
        );
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_t4() {
        let oup = exec_target(
            TARGET_EXE_PATH,
            &[
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
        assert_eq!(oup.status.success(), true);
    }
    #[test]
    fn test_exec() {
        let oup = exec_target(TARGET_EXE_PATH, &["/bin/true"]);
        assert_eq!(oup.stdout, "");
        let lines: Vec<_> = oup.stderr.lines().collect();
        // "pid: 817768, rss: 4ki, swap: 0ki, total: 4ki, comm: true"
        //assert_eq!(lines[0], "");
        assert_text_match!(
            lines[0],
            r"^pid: \d+, rss: +\d+ki, swap: +\d+ki, total: +\d+ki, comm: true$"
        );
        assert_eq!(oup.status.success(), true);
    }
} // mod test_1
