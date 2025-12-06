use once_cell::sync::Lazy;
use std::sync::Mutex;

#[warn(missing_docs)]
#[derive(Debug)]
pub enum TreeError {
    ClapHelp(String),
    Io(std::io::Error),
}

impl std::fmt::Display for TreeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            TreeError::ClapHelp(message) => write!(f, "{}", message),
            TreeError::Io(error) => write!(f, "{}", error),
        }
    }
}

/// Defines the type of function to be called in the command tree.
pub type NodeFn<P, R> = Box<dyn FnOnce(&clap::ArgMatches, Option<P>) -> R>;

/// Represents a command in the command tree.
pub trait Node<P, R> {
    fn name(&self) -> &str;
    fn command(&self) -> clap::Command;
    fn subcommands(&self) -> Vec<Box<dyn Node<P, R>>>;
    fn f(&self) -> NodeFn<P, R>;
}

/// A lazily initialized vector of dynamically dispatched nodes. Intended to serve as the type definition of a static vector of subcommands for a node.
pub type LazyNodeVec<P, R> = Lazy<Mutex<Vec<Box<dyn Node<P, R> + Send>>>>;

/// Get the function of a node provided the node name. If there are identical
/// node names, the node that is first encountered in the vector will have its
/// function returned.
fn find_f<P, R>(nodes: Vec<Box<dyn Node<P, R>>>, name: &str) -> Option<NodeFn<P, R>> {
    nodes.iter().find(|c| c.name() == name).map(|c| c.f())
}

pub fn run_tree<P, R>(
    node: Box<dyn Node<P, R>>,
    parent_matches: Option<&clap::ArgMatches>,
    params: Option<P>,
) -> Result<R, TreeError> {
    let mut command = node.command();
    let matches = if let Some(m) = parent_matches {
        m
    } else {
        &command.clone().get_matches()
    };

    if let Some((name, arg_matches)) = matches.subcommand() {
        match find_f(node.subcommands(), name) {
            Some(f) => Ok(f(arg_matches, params)),
            None => todo!(),
        }
    } else {
        match command.print_long_help() {
            Ok(_) => Err(TreeError::ClapHelp(String::from("No command executed."))),
            Err(e) => Err(TreeError::Io(e)),
        }
    }
}
