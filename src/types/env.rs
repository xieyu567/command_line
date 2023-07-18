use clap::ValueEnum;

#[derive(Eq, PartialEq, Hash, Debug, Clone, ValueEnum)]
pub(crate) enum Env {
    Dev,
    Uat,
    Prod,
}
