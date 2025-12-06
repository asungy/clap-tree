struct Params {
    pub message: String,
}

type CmdResult = Result<(), Box<dyn std::error::Error>>;

type CmdNode = Box<dyn clap_tree::Node<Params, CmdResult>>;

mod root {
    use super::{CmdNode, CmdResult, Params};

    const NAME: &str = "git";

    pub fn node() -> CmdNode {
        struct C;

        impl clap_tree::Node<Params, CmdResult> for C {
            fn name(&self) -> &str {
                todo!()
            }

            fn command(&self) -> clap::Command {
                clap::Command::new(NAME).about("I'm a git dummy!")
            }

            fn subcommands(&self) -> Vec<Box<dyn clap_tree::Node<Params, CmdResult>>> {
                Vec::<CmdNode>::new()
            }

            fn f(&self) -> clap_tree::NodeFn<Params, CmdResult> {
                Box::new(
                    |_matches: &clap::ArgMatches, _params: Option<Params>| -> CmdResult {
                        println!("hello git!");
                        Ok(())
                    },
                )
            }
        }

        Box::new(C {})
    }
}

fn main() {
    _ = clap_tree::run_tree(root::node(), None, None);
}
