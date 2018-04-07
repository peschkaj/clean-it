extern crate walkdir;

use std::env;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let args: Vec<_> = env::args().collect();

    if args.len() == 1 {
        println!("Usage: clean-it Term [Project]");
        println!("\nExamples:\n\tclean-it Winter2018\n\tclean-it Winter2018 P2");
    } else if args.len() == 2 {
        let term = &args[1];
        clean_term(term);
    } else {
        let term = &args[1];
        let project = &args[2];
        clean_project(term, project);
    }
}

fn clean_term(term: &String) {
    println!("Cleaning up for the term {:?}", term);
    // Don't care about path not existing because if the current directory
    // doesn't exist, we can't very well clean up...
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push(term);
    let term_root = Path::new(&current_dir);

    match env::set_current_dir(&term_root) {
        Ok(_) => clean_projects_in_term(term_root),
        Err(_) => panic!("Couldn't `cd` to {:?}", term),
    }
}

fn clean_projects_in_term(term: &Path) {
    // iterates over all entries and ignores any errors
    for entry in WalkDir::new(term.to_str().unwrap())
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() {
            println!("{}", entry.path().display());
        }
    }
}

fn clean_project(term: &String, project: &String) {
    println!("Cleaning up project {:?} for term {:?}", project, term);
}
