#[macro_use]
extern crate log;

use std::env;
use std::process::Command;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn wlc_terminate() {
    unimplemented!();
}

fn cwm_terminate(exit_code: i8) {
    let terminate_request = true;
    let exit_value = exit_code;
    wlc_terminate();
}

fn sig_handler(signal: i8) {
    //close_views(&mut root_container);
    //cwm_terminate(EXIT_SUCCESS);
}

// fn wlc_log_handler(log_type: WLCLogType, s: &str) {
//     match log_type {
//         WLCLogType::Error => cwm_log(L_ERROR, "[wlc] {}", s),
//         WLCLogType::Warn => cwm_log(L_INFO, "[wlc] {}", s),
//         _ => cwm_log(L_DEBUG, "[wlc] {}", s),
//     }
// }

fn detect_proprietary() {
    let mut proc_h = File::open("/proc/modules");
    match proc_h {
        Err(_) => return,
        Ok(f) => {
            let mut buf_f = BufReader::new(f);

            for line in buf_f.lines() {
                let line = line.unwrap();
                if line.contains("nvidia") {
                    error!("Warning: Proprietary \
                             Nvidia drivers are \
                             not known to support \
                             wayland.");
                    break;
                }
                if line.contains("fglrx") {
                    error!("Warning: Proprietary AMD \
                             drivers are not known to \
                             support Wayland.");
                    break;
                }
            }
        }
    }
}

//fn run_as_ipc_client(command: &str, socket_path: &str) {
//    let socketfd = ipc_open_socket(socket_path);
//    let len = command.len();
//    let resp = ipc_single_command(socketfd, IPC_COMMAND, command, &len);
//    log!("{}", resp);
//    close(socketfd);
//}

fn log_env() {
    let log_vars = vec!["PATH",
                        "LD_LOAD_PATH",
                        "LD_PRELOAD_PATH",
                        "LD_LIBRARY_PATH",
                        "CWM_CURSOR_THEME",
                        "CWM_CURSOR_SIZE",
                        "CWMSOCK",
                        "WLC_DRM_DEVICE",
                        "WLC_SHM",
                        "WLC_OUTPUTS",
                        "WLC_XWAYLAND",
                        "WLC_LIBINPUT",
                        "WLC_REPEAT_DELAY",
                        "WLC_REPEAT_RATE",
                        "XKB_DEFAULT_RULES",
                        "XKB_DEFAULT_MODEL",
                        "XKB_DEFAULT_LAYOUT",
                        "XKB_DEFAULT_VARIANT",
                        "XKB_DEFAULT_OPTIONS"];
    for var in log_vars {
        info!("{}={}", var, env::var(var).unwrap());
    }
}

fn log_distro() {
    let paths = vec!["/etc/lsb-release",
                     "/etc/os-release",
                     "/etc/debian_version",
                     "/etc/redhat-release",
                     "/etc/gentoo-release"];
    for path in paths {
        let f = File::open(path);
        match f {
            Err(_) => {
                continue;
            }
            Ok(h) => {
                info!("Contents of {}", path);
                let buf_h = BufReader::new(h);

                for line in buf_h.lines() {
                    let line = line.unwrap();
                    info!("{}", line);
                }
            }
        }
    }
}

fn log_kernel() {
    let uname = Command::new("uname")
        .arg("-a")
        .output();
    match uname {
        Err(_) => {
            info!("Unable to determine kernel version");
            return;
        }
        Ok(o) => {
            info!("{}", String::from_utf8_lossy(&o.stdout));
        }
    }
}

fn main() {
    let verbose = 0;
    let debug = 0;
    let validate = 0;
/*
    let long_options = vec![vec!["help", no_argument, NULL, 'h'],
                            vec!["config", required_argument, NULL, 'c'],
                            vec!["validate", no_argument, NULL, 'C'],
                            vec!["debug", no_argument, NULL, 'd'],
                            vec!["version", no_argument, NULL, 'v'],
                            vec!["verbose", no_argument, NULL, 'V'],
                            vec!["get-socketpath", no_argument, NULL, 'p'],
                            vec![0, 0, 0, 0]];

    let config_path: Option<&str> = None;
    let usage = r"Usage: sway [options] [command]

  -h, --help             Show help message and quit.
  -c, --config <config>  Specify a config file.
  -C, --validate         Check the validity of the config file, then exit.
  -d, --debug            Enables full logging, including debug information.
  -v, --version          Show the version number and quit.
  -V, --verbose          Enables more verbose logging.
      --get-socketpath   Gets the IPC socket path and prints it, then exits.

";
    */

}
