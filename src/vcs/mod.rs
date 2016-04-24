use yaml_rust::Yaml;

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

impl Repo {
    pub fn from_hash(hash: (&Yaml, &Yaml)) -> Repo {
        Repo {
            path: hash.0.as_str().unwrap().to_string(),
            kind: RepoKind::Git,
        }
    }
}
