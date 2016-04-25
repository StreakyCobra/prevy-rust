pub struct Update;

use std::sync::mpsc::channel;

use threadpool::ThreadPool;

use commands::Cmd;
use vcs::Repo;


impl Cmd for Update {
    fn run(repos: &[Repo]) -> Box<Fn() -> ()> {
        let pool = ThreadPool::new(1);
        let (tx, rx) = channel();

        for repo in repos {
            println!("{:?}", repo);
            let tx = tx.clone();
            pool.execute(move || {
                // repo.kind.clone_repo(&repo);
                tx.send(0).unwrap();
            });
        }

        for val in rx.iter().take(repos.len()) {
            println!("{:?}", val);
        }
        Box::new(move || println!("{}", "Hello"))
    }
}
