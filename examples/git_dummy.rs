pub mod prelude {
    pub struct Params {}
    pub type CmdResult = Result<(), Box<dyn std::error::Error>>;
    pub type CmdNode = Box<dyn clap_tree::Node<Params, CmdResult>>;
}

mod root {
    mod init {
        use crate::prelude::*;

        const NAME: &str = "init";

        pub fn node() -> CmdNode {
            struct C;

            impl clap_tree::Node<Params, CmdResult> for C {
                fn name(&self) -> &str {
                    NAME
                }

                fn command(&self) -> clap::Command {
                    clap::Command::new(NAME).about("fake git init")
                }

                fn children_nodes(&self) -> Vec<Box<dyn clap_tree::Node<Params, CmdResult>>> {
                    Vec::<CmdNode>::new()
                }

                fn f(&self) -> clap_tree::NodeFn<Params, CmdResult> {
                    Box::new(|_: &clap::ArgMatches, _: Option<Params>| -> CmdResult {
                        println!("not really initializing a git repo");
                        Ok(())
                    })
                }
            }

            Box::new(C {})
        }
    }

    mod branch {
        use crate::prelude::*;

        const NAME: &str = "branch";

        pub fn node() -> CmdNode {
            struct C;

            impl clap_tree::Node<Params, CmdResult> for C {
                fn name(&self) -> &str {
                    NAME
                }

                fn command(&self) -> clap::Command {
                    clap::Command::new(NAME).about("fake git branch")
                }

                fn children_nodes(&self) -> Vec<Box<dyn clap_tree::Node<Params, CmdResult>>> {
                    Vec::<CmdNode>::new()
                }

                fn f(&self) -> clap_tree::NodeFn<Params, CmdResult> {
                    Box::new(|_: &clap::ArgMatches, _: Option<Params>| -> CmdResult {
                        println!("real branches are found on real trees");
                        Ok(())
                    })
                }
            }

            Box::new(C {})
        }
    }

    mod remote {
        // LEFT OFF
        mod add {
            use crate::prelude::*;
        }

        mod remove {
            use crate::prelude::*;
        }

        use crate::prelude::*;
    }

    use crate::prelude::*;

    const NAME: &str = "git";

    pub fn node() -> CmdNode {
        struct C;

        impl clap_tree::Node<Params, CmdResult> for C {
            fn name(&self) -> &str {
                NAME
            }

            fn command(&self) -> clap::Command {
                clap::Command::new(NAME)
                    .about("fake git")
                    .subcommands(clap_tree::map_to_clap(self.children_nodes()))
                    .subcommand_required(true)
            }

            fn children_nodes(&self) -> Vec<Box<dyn clap_tree::Node<Params, CmdResult>>> {
                vec![init::node(), branch::node()]
            }

            fn f(&self) -> clap_tree::NodeFn<Params, CmdResult> {
                Box::new(
                    |_matches: &clap::ArgMatches, _params: Option<Params>| -> CmdResult {
                        unreachable!("This function is not expected to be called.")
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
