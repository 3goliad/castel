use getopts::{Options, Matches};
use std::env;

/// Where the shell intends to read commands from
#[derive(Debug)]
pub enum Source {
    Interactive(String),
    File(String),
    StandardInput(String),
    CommandString(String),
}

/// Options contains the current status of all the options
/// that can be modified with command line arguments or the set command
#[derive(Debug)]
pub struct Opts {
    pub allexport: bool,
    pub errexit: bool,
    pub ignoreeof: bool,
    pub monitor: bool,
    pub noclobber: bool,
    pub noglob: bool,
    pub noexec: bool,
    pub nolog: bool,
    pub notify: bool,
    pub nounset: bool,
    pub verbose: bool,
    pub vi: bool,
    pub xtrace: bool,
    pub locate_on_definition: bool,
}

//default is all false
pub fn from_cmdline_args() -> (Source, Opts) {
    let args = env::args().collect::<Vec<String>>();
    let (app_name, args) = args.split_first().unwrap();
    let mut other_args: String = String::new();

    let mut opts = Options::new();
    opts.optflag("c", "", "command_string");
    opts.optflag("s", "", "standard input");
    opts.optflag("a", "", "allexport");
    opts.optflag("b", "", "notify");
    opts.optflag("C", "", "noclobber");
    opts.optflag("e", "", "errexit");
    opts.optflag("f", "", "noglob");
    opts.optflag("h", "", "locate_on_definition");
    opts.optflag("m", "", "monitor");
    opts.optflag("n", "", "noexec");
    opts.optflag("u", "", "nounset");
    opts.optflag("v", "", "verbose");
    opts.optflag("x", "", "xtrace");
    opts.optmulti("o", "", "set option", "OPTION");

    let invalid_option = || {
        let brief = format!("Usage: {} FILE [options]", app_name);
        print!("{}", opts.usage(&brief));
        panic!();
    };

    let opts_matches: Matches = opts.parse(args).expect("Bad Options!");

    let mut allexport: bool = opts_matches.opt_present("a");
    let mut errexit: bool = opts_matches.opt_present("e");
    let mut monitor: bool = opts_matches.opt_present("m");
    let mut noclobber: bool = opts_matches.opt_present("C");
    let mut noglob: bool = opts_matches.opt_present("f");
    let mut noexec: bool = opts_matches.opt_present("n");
    let mut notify: bool = opts_matches.opt_present("b");
    let mut nounset: bool = opts_matches.opt_present("u");
    let mut verbose: bool = opts_matches.opt_present("v");
    let mut xtrace: bool = opts_matches.opt_present("x");
    let mut locate_on_definition: bool = opts_matches.opt_present("h");
    let mut vi: bool = false;
    let mut ignoreeof: bool = false;
    let mut nolog: bool = false;
    let stdset: bool = opts_matches.opt_present("s");
    let cmdset: bool = opts_matches.opt_present("c");
    let intset: bool = opts_matches.opt_present("i");

    for arg in opts_matches.opt_strs("o") {
        match arg.as_str() {
            "allexport" => allexport = true,
            "errexit" => errexit = true,
            "ignoreeof" => ignoreeof = true,
            "monitor" => monitor = true,
            "noclobber" => noclobber = true,
            "noglob" => noglob = true,
            "noexec" => noexec = true,
            "nolog" => nolog = true,
            "notify" => notify = true,
            "nounset" => nounset = true,
            "verbose" => verbose = true,
            "vi" => vi = true,
            "xtrace" => xtrace = true,
            _ => invalid_option(),
        }
    }

    // TODO there has to be an idiomatic way to do this
    let mut unset_next: bool = false;
    for arg in opts_matches.free {
        if unset_next {
            let mut skip: bool = true;
            match arg.as_ref() {
                "allexport" => allexport = false,
                "errexit" => errexit = false,
                "ignoreeof" => ignoreeof = false,
                "monitor" => monitor = false,
                "noclobber" => noclobber = false,
                "noglob" => noglob = false,
                "noexec" => noexec = false,
                "nolog" => nolog = false,
                "notify" => notify = false,
                "nounset" => nounset = false,
                "verbose" => verbose = false,
                "vi" => verbose = false,
                "xtrace" => xtrace = false,
                _ => skip = false,
            }
            unset_next = false;
            if skip {continue;}
        }

        match arg.as_ref() {
            "+o" => unset_next = true,
            "+a" => allexport = false,
            "+b" => notify = false,
            "+C" => noclobber = false,
            "+e" => errexit = false,
            "+f" => noglob = false,
            "+h" => locate_on_definition = false,
            "+m" => monitor = false,
            "+n" => noexec = false,
            "+u" => nounset = false,
            "+v" => verbose = false,
            "+x" => xtrace = false,
            other => {
                if other.as_bytes()[0] == '+' as u8 {
                    for c in other.chars() {
                        match c {
                            'a' => allexport = false,
                            'b' => notify = false,
                            'C' => noclobber = false,
                            'e' => errexit = false,
                            'f' => noglob = false,
                            'h' => locate_on_definition = false,
                            'm' => monitor = false,
                            'n' => noexec = false,
                            'u' => nounset = false,
                            'v' => verbose = false,
                            'x' => xtrace = false,
                            _ => invalid_option(),
                        }
                    }
                } else {
                    other_args.push_str(other);
                }
            }
        }
    }

    let opts = Opts {
        allexport: allexport,
        errexit: errexit,
        ignoreeof: ignoreeof,
        monitor: monitor,
        noclobber: noclobber,
        noglob: noglob,
        noexec: noexec,
        nolog: nolog,
        notify: notify,
        nounset: nounset,
        verbose: verbose,
        vi: vi,
        xtrace: xtrace,
        locate_on_definition: locate_on_definition,
    };

    let otrset = other_args == "";
    if !cmdset && !intset && (!stdset | otrset) {
        return (Source::StandardInput(other_args), opts);
    }
    if !stdset {
        if !intset && otrset {
            if !cmdset {
                return (Source::File(other_args), opts);
            } else {
                return (Source::CommandString(other_args), opts);
            }
        } else {
            return (Source::Interactive(other_args), opts);
        }
    }
    invalid_option();
    unreachable!();
}
