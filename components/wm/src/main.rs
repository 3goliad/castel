extern crate nix;
#[macro_use]
extern crate log;
extern crate env_logger;
#[macro_use]
extern crate clap;

use nix::unistd::{getgid, setgid, getuid, setuid, geteuid, getegid};

use std::env;
use std::process::{exit, Command};
use std::fs::File;
use std::io::{BufRead, BufReader};

use extensions::*;

fn wlc_terminate() {
  unimplemented!();
}

fn cwm_terminate(exit_code: i8) {
  let terminate_request = true;
  let exit_value = exit_code;
  wlc_terminate();
}

fn sig_handler(signal: i8) {
  // close_views(&mut root_container);
  // cwm_terminate(EXIT_SUCCESS);
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
          error!("Warning: Proprietary Nvidia drivers are not known to \
                  support wayland.");
          break;
        }
        if line.contains("fglrx") {
          error!("Warning: Proprietary AMD drivers are not known to support \
                  Wayland.");
          break;
        }
      }
    }
  }
}

// fn run_as_ipc_client(command: &str, socket_path: &str) {
//    let socketfd = ipc_open_socket(socket_path);
//    let len = command.len();
//    let resp = ipc_single_command(socketfd, IPC_COMMAND, command, &len);
//    log!("{}", resp);
//    close(socketfd);
//

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
  env_logger::init().unwrap();
  let matches = clap_app!(
        app =>
            (version: crate_version!())
            (author: crate_authors!())
            (about: "cwm")
            (@arg config: -c --config +takes_value
             "Specify a config file.")
            (@arg validate: -C --validate
             "Check the validity of the config file \
              and exit")
            (@arg debug: -d --debug
             "Enables debug logging.")
            (@arg verbose: -v --verbose
             "Enables more verbose logging.")
            (@arg get_socketpath: --get-socketpath
             "Gets the IPC socket path and prints it, \
              then exits.")
    )
    .get_matches();
  let config_path = matches.value_of("config");
  let validate = matches.is_present("validate");
  let debug = matches.is_present("debug");
  let verbose = matches.is_present("verbose");

  if matches.is_present("get_socketpath") {
    match env::var("CWMSOCK") {
      Err(_) => {
        println!("cwm socket not detected");
        exit(1);
      }
      Ok(s) => {
        println!("{}", s);
        exit(0);
      }
    }
  }

  // TODO: when is IPC enabled?
  let ipc_client = false;
  if ipc_client {
    if env::args().count() != 1 {
      error!("Don't use options with the IPC client");
      exit(1);
    }
    if getuid() != geteuid() || getgid() != getegid() {
      if setgid(getgid()) != 0 {
        error!("Unable to drop root");
        exit(1);
      }
      if setuid(getuid()) != 0 {
        error!("Unable to drop root");
        exit(1);
      }
    }
    if setuid(0) != -1 {
      error!("Root privileges can be restored.");
      exit(1);
    }

    let socket_path = env::var("CWMSOCK")
      .expect("Unable to retrieve socket path");
    // run_as_ipc_client(command, socket_path);
  }

  wlc_log_set_handler(wlc_log_handler);
  detect_proprietary();

  let input_devices = create_list();

  register_wlc_handlers();
  if !wlc_init() {
    // return 1;
  }
  register_extensions();

  // TODO signal(SIGTERM, sig_handler)
  // TODO signal(SIGPIPE, SIG_IGN)
  info!("Starting cwm version {}", crate_version!());
  log_kernel();
  log_distro();
  log_env();

  init_layout();

  ipc_init();

  match load_main_config(config_path, false) {
    true if validate => { exit(0); },
    false if validate => { exit(1); },
    true => {()},
    false => { cwm_terminate(EXIT_FAILURE);},
  }

  if !terminate_request {
    wlc_run();
  }

  ipc_terminate();

  return exit_value;
}
