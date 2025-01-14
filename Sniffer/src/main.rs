/* TODO :
 *          - Improve coloring
 */
use std::{env, path::PathBuf};
use toolkit::{get_arg_opts, print_headers, show_files, Argopts};
fn main() {
    let opts: Argopts = get_arg_opts();
    let mut path = PathBuf::from(env::current_dir().unwrap());
    if opts.explicit_path {
        path.push(opts.exp_path.to_string());
    } else {
        path.push(&opts.exp_path);
    }
    if path.is_dir() {
        print_headers(&path);
        match opts.tree {
            true => show_files(opts.hidden, path, 0, true),
            false => show_files(opts.hidden, path, 0, false),
        }
    }
    print!("\n");
}
