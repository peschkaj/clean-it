extern crate walkdir;

use std::env;
use std::path::Path;
use std::process::Command;
use walkdir::WalkDir;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: clean-it Term [Project]");
        println!("\tTerm should be a path to a term (e.g. ~/Winter2018)");
        println!("\nExamples:\n\tclean-it Winter2018\n\tclean-it Winter2018 P2");
    } else if args.len() == 2 {
        let term = &args[1];
        let mut root = env::current_dir().unwrap();
        root.push(term);
        clean_it(&Path::new(&root));
    } else {
        let term = &args[1];
        let project = &args[2];
        let mut root = env::current_dir().unwrap();
        root.push(term);
        root.push(project);
        clean_it(&Path::new(&root));
    }
}

fn clean_it(path: &Path) {
    for entry in WalkDir::new(path.to_str().unwrap())
        .max_depth(4) // should be the max depth from term to dist
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() && entry.path().to_str().unwrap().ends_with("dist") {
            println!("{}", entry.path().display());
            make_clean(entry.path());
        }
    }
}

fn make_clean(d: &Path) {
    let status = Command::new("make")
        .current_dir(&d)
        .arg("clean")
        .status()
        .expect(&format!("unable to `make clean` in {}", d.display()));

    println!(
        "\n\t`make clean` in {} exited with status {}\n\n",
        d.display(),
        status
    );
}
