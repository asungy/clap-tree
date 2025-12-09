//! A personal helper crate to help me build CLIs. I do not recommend others to use it.

#![warn(missing_docs)]
/// Represents an error that can occur with a tree Node.
#[derive(Debug)]
pub enum TreeError {
    /// Not a real error. This error occurs when no subcommand gets matched and
    /// the help menu is displayed.
    ClapHelp(String),
    /// IO error.
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

impl std::error::Error for TreeError {}

/// Defines the type of function to be call before a node's `f` function (if it
/// exists).
pub type NodePreFn<P> = Box<dyn FnOnce(&clap::ArgMatches, Option<P>) -> Option<P>>;

/// Defines the type of function to be called in the command tree.
pub type NodeFn<P, R> = Box<dyn FnOnce(&clap::ArgMatches, Option<P>) -> Result<R, TreeError>>;

/// Represents a command in the command tree.
pub trait Node<P, R> {
    /// The clap ID name for the command node.
    fn name(&self) -> &str;
    /// The clap command.
    fn command(&self) -> clap::Command;
    /// Subcommand nodes.
    fn children_nodes(&self) -> Vec<Box<dyn Node<P, R>>>;
    /// A function used to transform custom parameters.
    fn pre_f(&self) -> Option<NodePreFn<P>>;
    /// The function to run when the node command is called.
    fn f(&self) -> Option<NodeFn<P, R>>;
}

/// Map nodes to their respective clap commands.
pub fn map_to_clap<P, R>(nodes: Vec<Box<dyn Node<P, R>>>) -> Vec<clap::Command> {
    nodes.iter().map(|c| c.command()).collect()
}

/// Run a parent node command.
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
        match find_f(node.children_nodes(), name) {
            Some(f) => {
                let params = match node.pre_f() {
                    Some(pre_f) => pre_f(matches, params),
                    None => params,
                };
                match f(arg_matches, params) {
                    Ok(result) => Ok(result),
                    Err(e) => Err(e),
                }
            }
            None => unreachable!("Could not find subcommand."),
        }
    } else {
        match command.print_long_help() {
            Ok(_) => Err(TreeError::ClapHelp(String::from("No command executed."))),
            Err(e) => Err(TreeError::Io(e)),
        }
    }
}

/// Get the function of a node provided the node name. If there are identical
/// node names, the node that is first encountered in the vector will have its
/// function returned.
fn find_f<P, R>(nodes: Vec<Box<dyn Node<P, R>>>, name: &str) -> Option<NodeFn<P, R>> {
    nodes
        .iter()
        .find(|c| c.name() == name)
        .map(|c| c.f())
        .unwrap_or(None)
}
