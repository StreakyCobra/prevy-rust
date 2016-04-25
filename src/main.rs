// External crates
extern crate ansi_term;
extern crate atty;
extern crate clap;
extern crate threadpool;
extern crate xdg_basedir;
extern crate yaml_rust;

// Project modules
mod context;
mod core;
mod vcs;

use threadpool::ThreadPool;
use std::sync::mpsc::channel;

use vcs::Vcs;
use vcs::git::Git;

/// Run prevy.
fn main() {
    // Build the context:
    // - Handle command line arguments
    // - Handle environment variables
    // - Parse the configuration
    // - Parse the workspace
    // - Prepare a display
    let ctx = context::build_context();

    // Switch to workspace root
    ctx.workspace.cd_root();

    let pool = ThreadPool::new(1);
    let (tx, rx) = channel();

    for repo in ctx.workspace.repos.clone() {
        println!("{:?}", repo);
        let tx = tx.clone();
        pool.execute(move || {
            Git::clone_repo(&repo);
            tx.send(0).unwrap();
        });
    }

    for val in rx.iter().take(ctx.workspace.repos.len()) {
        println!("{:?}", val);
    }
}
