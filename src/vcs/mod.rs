// ------------------------------------------------------------------------- //
// Structures                                                                //
// ------------------------------------------------------------------------- //

#[derive(Clone, Debug)]
pub enum RepoKind {
    Git,
}

#[derive(Clone, Debug)]
pub struct Repo {
    pub path: String,
    pub kind: RepoKind,
}
