pub mod update;

use vcs::Repo;

trait Cmd {
    fn run(repos: &[Repo]) -> Box<Fn() -> ()>;
}
