// ------------------------------------------------------------------------- //
// Imports                                                                   //
// ------------------------------------------------------------------------- //

// Standard libraries imports
use std::process;
use std::result::Result as StdResult;

// Project imports
use core::display;

// ------------------------------------------------------------------------- //
// Structures                                                                //
// ------------------------------------------------------------------------- //

#[derive(Clone, Debug)]
enum RepoKind {
    Git,
}

#[derive(Clone, Debug)]
struct Repo {
    pub path: String,
    pub kind: RepoKind,
}
