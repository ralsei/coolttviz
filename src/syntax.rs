use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum IdentType {
    User,
    Machine,
    Anonymous,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Ident {
    pub ty: IdentType,
    pub name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Cell {
    pub names: Vec<Ident>,
    pub ty: Box<ConcreteSyntax>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ConcreteSyntax {
    Var(Ident),
    Let(Box<ConcreteSyntax>, Ident, Box<ConcreteSyntax>),
    Nat,
    Suc(Box<ConcreteSyntax>),
    Lit(u32),
    Circle,
    Base,
    Loop(Box<ConcreteSyntax>),
    Pi(Vec<Cell>, Box<ConcreteSyntax>),
    Lam(Vec<Ident>, Box<ConcreteSyntax>),
    Ap(Box<ConcreteSyntax>, Vec<Box<ConcreteSyntax>>),
    Sg(Vec<Cell>, Box<ConcreteSyntax>),
    Pair(Box<ConcreteSyntax>, Box<ConcreteSyntax>),
    Fst(Box<ConcreteSyntax>),
    Snd(Box<ConcreteSyntax>),
    Type,
    Hole(Option<String>, Option<Box<ConcreteSyntax>>),
    BoundaryHole(Option<Box<ConcreteSyntax>>),
    Underscore,
    Dim,
    Cof,
    CofEq(Box<ConcreteSyntax>, Box<ConcreteSyntax>),
    CofLe(Box<ConcreteSyntax>, Box<ConcreteSyntax>),
    Join(Vec<Box<ConcreteSyntax>>),
    Meet(Vec<Box<ConcreteSyntax>>),
    CofBoundary(Box<ConcreteSyntax>),
    Prf(Box<ConcreteSyntax>),
    CofSplit(Vec<(Box<ConcreteSyntax>, Box<ConcreteSyntax>)>),
    Coe(
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
    ),
    TopC,
    BotC,
    HCom(
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
    ),
    HFill(
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
    ),
    Com(
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
        Box<ConcreteSyntax>,
    ),
    Cap(Box<ConcreteSyntax>),
}
