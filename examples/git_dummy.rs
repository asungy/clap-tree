pub mod prelude {
    pub struct Params {
        pub message: String,
    }
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

                fn pre_f(&self) -> Option<clap_tree::NodePreFn<Params>> {
                    None
                }

                fn f(&self) -> Option<clap_tree::NodeFn<Params, CmdResult>> {
                    Some(Box::new(
                        |_: &clap::ArgMatches,
                         params: Option<Params>|
                         -> Result<CmdResult, clap_tree::TreeError> {
                            match params {
                                Some(p) => {
                                    println!("message: {}", p.message);
                                }
                                None => {}
                            };

                            println!("not really initializing a git repo");
                            Ok(Ok(()))
                        },
                    ))
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

                fn pre_f(&self) -> Option<clap_tree::NodePreFn<Params>> {
                    None
                }

                fn f(&self) -> Option<clap_tree::NodeFn<Params, CmdResult>> {
                    Some(Box::new(
                        |_: &clap::ArgMatches,
                         params: Option<Params>|
                         -> Result<CmdResult, clap_tree::TreeError> {
                            match params {
                                Some(p) => {
                                    println!("message: {}", p.message);
                                }
                                None => {}
                            };

                            println!("real branches are found on real trees");
                            Ok(Ok(()))
                        },
                    ))
                }
            }

            Box::new(C {})
        }
    }

    mod remote {
        mod add {
            use crate::prelude::*;

            const NAME: &str = "add";

            pub fn node() -> CmdNode {
                struct C;

                impl clap_tree::Node<Params, CmdResult> for C {
                    fn name(&self) -> &str {
                        NAME
                    }

                    fn command(&self) -> clap::Command {
                        clap::Command::new(NAME).about("fake git remote add")
                    }

                    fn children_nodes(&self) -> Vec<Box<dyn clap_tree::Node<Params, CmdResult>>> {
                        Vec::<CmdNode>::new()
                    }

                    fn pre_f(&self) -> Option<clap_tree::NodePreFn<Params>> {
                        None
                    }

                    fn f(&self) -> Option<clap_tree::NodeFn<Params, CmdResult>> {
                        Some(Box::new(
                        |_: &clap::ArgMatches,
                         params: Option<Params>|
                         -> Result<CmdResult, clap_tree::TreeError> {
                                match params {
                                    Some(p) => {
                                        println!("message: {}", p.message);
                                    }
                                    None => {}
                                };

                                println!("trust me I'm not adding anything.");
                                Ok(Ok(()))
                            },
                        ))
                    }
                }

                Box::new(C {})
            }
        }

        mod remove {
            use crate::prelude::*;

            const NAME: &str = "remove";

            pub fn node() -> CmdNode {
                struct C;

                impl clap_tree::Node<Params, CmdResult> for C {
                    fn name(&self) -> &str {
                        NAME
                    }

                    fn command(&self) -> clap::Command {
                        clap::Command::new(NAME).about("fake git remote remove")
                    }

                    fn children_nodes(&self) -> Vec<Box<dyn clap_tree::Node<Params, CmdResult>>> {
                        Vec::<CmdNode>::new()
                    }

                    fn pre_f(&self) -> Option<clap_tree::NodePreFn<Params>> {
                        None
                    }

                    fn f(&self) -> Option<clap_tree::NodeFn<Params, CmdResult>> {
                        Some(Box::new(
                        |_: &clap::ArgMatches,
                         params: Option<Params>|
                         -> Result<CmdResult, clap_tree::TreeError> {
                                match params {
                                    Some(p) => {
                                        println!("message: {}", p.message);
                                    }
                                    None => {}
                                };

                                println!("just kidding, nothing removed!");
                                Ok(Ok(()))
                            },
                        ))
                    }
                }

                Box::new(C {})
            }
        }

        use crate::prelude::*;

        const NAME: &str = "remote";

        pub fn node() -> CmdNode {
            struct C;

            impl clap_tree::Node<Params, CmdResult> for C {
                fn name(&self) -> &str {
                    NAME
                }

                fn command(&self) -> clap::Command {
                    clap::Command::new(NAME)
                        .about("fake git remote")
                        .subcommands(clap_tree::map_to_clap(self.children_nodes()))
                        .subcommand_required(true)
                }

                fn children_nodes(&self) -> Vec<Box<dyn clap_tree::Node<Params, CmdResult>>> {
                    vec![add::node(), remove::node()]
                }

                fn pre_f(&self) -> Option<clap_tree::NodePreFn<Params>> {
                    None
                }

                fn f(&self) -> Option<clap_tree::NodeFn<Params, CmdResult>> {
                    Some(Box::new(
                        |matches: &clap::ArgMatches,
                         params: Option<Params>|
                         -> Result<CmdResult, clap_tree::TreeError> {
                            clap_tree::run_tree(Box::new(C {}), Some(matches), params)
                        },
                    ))
                }
            }

            Box::new(C {})
        }
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
                    .args(vec![
                        clap::Arg::new("message")
                            .short('m')
                            .long("message")
                            .value_parser(clap::value_parser!(String)),
                    ])
                    .subcommands(clap_tree::map_to_clap(self.children_nodes()))
                    .subcommand_required(true)
            }

            fn children_nodes(&self) -> Vec<Box<dyn clap_tree::Node<Params, CmdResult>>> {
                vec![init::node(), branch::node(), remote::node()]
            }

            fn pre_f(&self) -> Option<clap_tree::NodePreFn<Params>> {
                Some(Box::new(
                    |matches: &clap::ArgMatches, params: Option<Params>| -> Option<Params> {
                        if let Some(message) = matches.get_one::<String>("message") {
                            let message = message.clone();
                            Some(Params { message })
                        } else {
                            params
                        }
                    },
                ))
            }

            fn f(&self) -> Option<clap_tree::NodeFn<Params, CmdResult>> {
                None
            }
        }

        Box::new(C {})
    }
}

fn main() {
    _ = clap_tree::run_tree(root::node(), None, None);
}
