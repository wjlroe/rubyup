extern crate binary_shim;
extern crate clap;

use clap::{App, SubCommand};
use std::fs::{File, read_dir};
use std::io::{Error, Read};
use std::path::Path;

type RunResult = Result<(), Error>;

fn find_a_ruby() -> Result<(), Box<Error>> {
    // TODO: _Actually_ find a Ruby, nice to have!
    if cfg!(target_os = "windows") {
        for direntry in read_dir("C:")? {
            let entryref = &direntry;
            if entryref
                .and_then(|entry| entry.file_type())
                .map(|file| file.is_dir())
                .unwrap_or(false)
            {
                println!("dir: {:?}", direntry);
            }
        }
        // let dirs = read_dir("C:")?
        //     .filter(|entry| {
        //                 entry
        //                     .and_then(|entry| entry.file_type())
        //                     .map(|file| file.is_dir())
        //                     .unwrap_or(false)
        //             });
        // for dir in dirs {
        //     println!("dir: {:?}", dir);
        // }
    }
    Ok(())
}

fn run_a_ruby() -> RunResult {
    // find a .ruby-version file
    let ruby_version_file = Path::new(".ruby-version");
    // if there is one, use that to find a ruby
    if ruby_version_file.exists() {
        println!(".ruby-version file exists");
        let mut ruby_version = String::new();
        let mut file = File::open(ruby_version_file)?;
        file.read_to_string(&mut ruby_version)?;
        println!("Ruby version from file: {}", ruby_version);
    } else {
        println!(".ruby-version file does not exist");
    }
    Ok(())
}

fn show_rubies() -> RunResult {
    // is a ruby selected by a .ruby-version file?
    // which ruby is that? on Windows we have i386 and x64
    // is it a "system" ruby or one installed and managed by rubyup?
    Ok(())
}

fn main() {
    let matches = App::new("rubyup")
        .version("1.0") // get the version from Cargo.toml?
        .author("Will Roe")
        .about("Manage your rubies")
        .subcommand(SubCommand::with_name("show")
                    .about("show the available and selected rubies"))
        .get_matches();

    find_a_ruby();
    if let Some(_) = matches.subcommand_matches("show") {
        let _ = show_rubies();
    } else {
        let _ = run_a_ruby();
    }
}
