use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum ChkTactic {
    SilentHole(Option<String>),
    UnleashHole(Option<String>),
    UnleashSynHole(Option<String>),
}

#[derive(Debug, Serialize)]
pub enum SynTactic {}
