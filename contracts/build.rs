extern crate core;

use std::fs;
use std::io::Write;

use ethers_solc::{Project, ProjectPathsConfig};

fn main() {
    // Configure the project with all its paths, solc, cache etc.
    let project = Project::builder()
        .paths(ProjectPathsConfig::hardhat(env!("CARGO_MANIFEST_DIR")).unwrap())
        .build()
        .unwrap();
    let output = project.compile().unwrap();

    if output.has_compiler_errors() || output.has_compiler_warnings() {
        // HACK: Bypass the parent cargo output capture and send directly to the tty, if available.
        // This allows solc to forward errors and warnings to the user.
        let mut tty = fs::OpenOptions::new().write(true).open("/dev/tty").ok();

        if let Some(tty) = &mut tty {
            for error in output.clone().output().errors.iter() {
                write!(tty, "{}", error).unwrap();
            }
            if output.has_compiler_errors() {
                panic!("Failed to build Solidity contracts");
            }
        } else {
            panic!("{:?}", output.output().errors);
        }
    }

    // Tell Cargo that if a source file changes, to rerun this build script.
    project.rerun_if_sources_changed();
}
