use std::env;
use std::fs;
use std::os::unix::fs::MetadataExt; // required for .dev()
use std::path::Path;
use std::process;

fn available_mount(path_str: &str, verbose: bool) -> bool {
    let path = Path::new(path_str);
    
    // equivalent to os.path.realpath
    let full_path = match fs::canonicalize(path) {
        Ok(p) => p,
        Err(_) => return false,
    };

    // get parent directory; if none, use the path itself
    let parent = full_path.parent().unwrap_or(&full_path);

    // get metadata (os.stat)
    let path_meta = fs::metadata(&full_path).ok();
    let parent_meta = fs::metadata(parent).ok();

    if let (Some(pm), Some(parm)) = (path_meta, parent_meta) {
        // .dev() is the equivalent of st_dev
        let available = pm.dev() == parm.dev();

        if verbose {
            eprintln!("path: {:?}, Device ID: {}", full_path, pm.dev());
            eprintln!("parent: {:?}, Device ID: {}", parent, parm.dev());
            eprintln!("available: {}", available);
        }
        available
    } else {
        false
    }
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    // filter for targets (non-flag arguments)
    let target = args.iter().find(|arg| !arg.starts_with('-'));
    let verbose = args.iter().any(|arg| arg == "-v");
    const EXIT_OK:i32 = 0;
    const EXIT_NOT_OK:i32 = 1;
    let exit_code = match target {
        Some(t) => {
            if available_mount(t, verbose) {
                eprintln!("mount available: {:?}", t);
                EXIT_OK
            } else {
                eprintln!("mount unavailable: {:?}", t);
                EXIT_NOT_OK
            }
        }
        None => { EXIT_NOT_OK }
    };
    process::exit(exit_code);
}
