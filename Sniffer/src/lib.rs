use chrono::{DateTime, Local};
use core::fmt;
use matcher::{get_color, BRIGHT_MAGENTA, BRIGHT_RED, RESET};
use std::{
    env::{self},
    fmt::Formatter,
    fs::{self, *},
    ops::Index,
    os::unix::fs::MetadataExt,
    path::PathBuf,
    process::exit,
};

mod matcher;

/*
 *
 * NOTE body of main method acessed by fn main()
 *
 *
 */

pub fn show_files(allowhidden: bool, path: PathBuf, mut recursive: u8, mut recursive_check: bool) {
    let dir = fs::read_dir(&path).unwrap();
    for f in dir {
        let entry = f.unwrap();
        let mut ent: Entry = Entry::new();
        ent.collectdata(entry);
        if (!ent.is_hidden() || allowhidden) && recursive == 0 {
            println!("{}", &ent);
        }
        if recursive >= 1 && (!ent.is_hidden() || allowhidden) {
            println!("{}", str_builder_recursive(&ent));
        }
        if recursive_check && ent.dir && (!ent.is_hidden() || allowhidden) {
            recursive += 1;
            recursive_check = false;
            let mut recpath = path.clone();
            recpath.push(&ent.filename);
            show_files(allowhidden.clone(), recpath, recursive, recursive_check);
            recursive = 0;
            recursive_check = true;
        }
    }
}

/*
 *
 *
 * NOTE RELATED TO FILE ANALYSIS
 *
 *
 */

pub struct Entry {
    path: PathBuf,
    filename: String,
    filesize: u64,
    lastmod: String,
    dir: bool,
    perm: String,
    ico: String,
    color: String,
    hidden: bool,
    is_sym: bool,
}
impl Entry {
    pub fn collectdata(&mut self, entry: DirEntry) {
        self.filesize = entry.metadata().unwrap().size();
        self.filename = entry.file_name().to_string_lossy().to_string();
        self.lastmod = (DateTime::from(entry.metadata().unwrap().modified().unwrap())
            as DateTime<Local>)
            .format("%H:%M:%S %d/%m/%Y")
            .to_string();
        self.dir = entry.metadata().unwrap().is_dir();
        if entry.metadata().unwrap().is_symlink() {
            self.path = fs::read_link(entry.path()).unwrap();
        } else {
            self.path = entry.path();
        }
        self.perm = match entry.metadata().unwrap().permissions().readonly() {
            true => "r1w0".to_string(),
            false => "r1w1".to_string(),
        };
        self.is_sym = entry.metadata().unwrap().is_symlink();
        self.ico = matcher::geticon(&self.filename, self.dir.clone(), self.is_sym.clone());
        self.color = get_color(&self.filename, self.dir.clone(), self.is_sym.clone());
        self.hidden = self.is_hidden();
    }
    pub fn new() -> Entry {
        Entry {
            filename: "Default".to_string(),
            filesize: 0,
            path: PathBuf::new(),
            lastmod: "None".to_string(),
            dir: false,
            perm: "Unknown".to_string(),
            ico: "".to_string(),
            color: "".to_string(),
            hidden: false,
            is_sym: false,
        }
    }
    fn is_hidden(&mut self) -> bool {
        self.filename.starts_with('.')
    }
}

impl fmt::Display for Entry {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let entry_trace: String = str_builder(
            &self.filename,
            &self.filesize,
            &self.lastmod,
            &self.perm,
            &self.ico,
            &self.color,
            &self,
        );
        write!(f, "{}", entry_trace)
    }
}

fn str_builder(
    name: &String,
    size: &u64,
    writetime: &String,
    perm: &String,
    ico: &String,
    color: &String,
    entry: &Entry,
) -> String {
    if !entry.is_sym {
        return format!(
            "{:6} {:20} {:10} {} {:3}{} {}",
            perm,
            writetime,
            size,
            color,
            ico,
            name,
            matcher::RESET
        );
    } else {
        return format!(
            "{:6} {:20} {:10} {} {:3}{} -> {} {}",
            perm,
            writetime,
            size,
            color,
            ico,
            name,
            entry.path.to_string_lossy().to_string(),
            matcher::RESET
        );
    }
}

fn str_builder_recursive(ent: &Entry) -> String {
    if !ent.is_sym {
        return format!(
            "{:6} {:20} {:10}  {}|{}{} {:3}{:15} {}",
            &ent.perm,
            &ent.lastmod,
            &ent.filesize,
            matcher::BRIGHT_CYAN,
            matcher::RESET,
            &ent.color,
            &ent.ico,
            &ent.filename,
            matcher::RESET
        );
    } else {
        return format!(
            "{:6} {:20} {:10}  {}|{}{} {:3}{} -> {} {}",
            &ent.perm,
            &ent.lastmod,
            &ent.filesize,
            matcher::BRIGHT_CYAN,
            matcher::RESET,
            &ent.color,
            &ent.ico,
            &ent.filename,
            &ent.path.to_string_lossy().to_string(),
            matcher::RESET
        );
    }
}
/*
 *
 *
 * NOTE THESE ARE IO METHODS
 *
 *
 */

pub fn print_headers(path: &PathBuf) {
    println!(
        "\n {}\n\n{}{:6} {:20} {:>10} {:3} {:15}\n{:6} {:20} {:>10} {:3} {:15}{}",
        path.to_string_lossy().to_string(),
        BRIGHT_MAGENTA,
        "PERMS",
        "LASTMODTIME",
        "SIZE",
        "",
        "NAME",
        "-----",
        "--------------------",
        "-----",
        "",
        "----------------",
        RESET,
    );
}

pub fn print_help() {
    println!("\n{}\n", "[SNIFF ARGUMENTS SCHEMA] sniff <opts> <path>");

    println!(
        "{:20} {:15}\n{:20} {:15}\n{:20} {:15}\n{:20} {:15}\n",
        "-man, -help, help",
        "Print this msg",
        "-h, -hid, -hidden",
        "Allow hidden files (dotfiles)",
        "-path, -p",
        "Path to list",
        "-r, -t, -tree",
        "Display a recursive tree of the subdirectories"
    );
}

/*
 *
 *
 * NOTE ARGUMENT PARSING METHODS
 *
 *
 */

pub struct Argopts {
    pub hidden: bool,
    pub explicit_path: bool,
    pub exp_path: String,
    pub tree: bool,
}
impl fmt::Display for Argopts {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "hidden: {}\nexplicit_path: {}\n",
            self.hidden, self.exp_path
        )
    }
}
const VALID_ARGS: [&str; 11] = [
    "-p", "-path", "-hid", "-hidden", "-man", "-h", "-hidden", "help", "-tree", "-r", "-t",
];

pub fn get_arg_opts() -> Argopts {
    let mut opts = Argopts {
        hidden: false,
        explicit_path: false,
        exp_path: ".".to_string(),
        tree: false,
    };
    let mut argv: Vec<String> = env::args().collect();
    argv.remove(0);
    if argv.len() == 0 {
        return opts;
    }
    let mut cont: usize = 0;
    for arg in &argv {
        match arg.as_str() {
            _ if arg.contains("-") && !VALID_ARGS.contains(&arg.as_str()) => {
                print_help();
                exit(0);
            }
            _ if !arg.contains("-") => {
                opts.explicit_path = true;
                opts.exp_path = arg.to_owned();
            }

            "-hidden" | "-hid" | "-h" => opts.hidden = true,

            "-path" | "-p" => {
                opts.explicit_path = true;
                opts.exp_path = argv.clone().index(cont + 1).to_owned();
            }
            "-man" | "-help" | "help" => {
                print_help();
                exit(0);
            }
            "-r" | "-tree" | "-t" => opts.tree = true,
            _ => (),
        }
        cont += 1;
    }
    opts
}
