use std::sync::mpsc::channel;

use threadpool::ThreadPool;

use commands::Cmd;
use vcs::Repo;

pub struct Update;

impl Cmd for Update {
    fn run(repos: Vec<Box<Repo + Send>>) -> Box<Fn() -> ()> {
        let pool = ThreadPool::new(1);
        let (tx, rx) = channel();

        for repo in repos {
            let tx = tx.clone();
            pool.execute(move || {
                repo.clone_repo();
                tx.send(0).unwrap();
            });
        }

        Box::new(move || println!("{}", "Hello"))
    }
}
