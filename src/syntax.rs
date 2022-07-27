use serde::{*, ser::SerializeSeq};

#[derive(Clone, Debug)]
pub enum Ident {
    Anon,
    User(Vec<String>),
    Machine(String),
}

#[derive(Clone, Debug, Serialize)]
pub struct Cell {
    pub names: Vec<Ident>,
    pub ty: Box<ConcreteSyntax>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Node {
    node: Box<ConcreteSyntax>,
}

#[derive(Clone, Debug, Serialize)]
pub struct Hole {
    pub name: Option<String>,
    pub silent: bool,
}

#[derive(Clone, Debug)]
pub enum ConcreteSyntax {
    Var(Ident),
    Lit(u32),
    Pi(Vec<Cell>, Node),
    Lam(Vec<Ident>, Node),
    Ap(Node, Vec<Node>),
    Sg(Vec<Cell>, Node),
    Type,
    Hole(Hole),
    BoundaryHole(Option<Node>),
    Underscore,
    Dim,
    Cof,
    CofEq(Node, Node),
    CofLe(Node, Node),
    Join(Vec<Node>),
    Meet(Vec<Node>),
    CofBoundary(Node),
    CofSplit(Vec<(Node, Node)>),
    TopC,
    BotC,
    HComChk(Node, Node, Node),
    HFillChk(Node, Node),
}

impl Node {
    pub fn new(cs: ConcreteSyntax) -> Node {
        Node { node: Box::new(cs) }
    }
}

use crate::Ident::*;
use crate::ConcreteSyntax::*;

// [HACK: Avery; 2022-07-25] Yojson and Serde have different representations,
// so we're stuck with this hellhole
//
// Maybe this could be a macro?
impl Serialize for Ident {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        match self {
            Anon => seq.serialize_element("Anon")?, 
            User(vs) => {
                seq.serialize_element("User")?;
                seq.serialize_element(vs)?;
            },
            Machine(st) => {
                seq.serialize_element("Machine")?;
                seq.serialize_element(st)?;
            },
        }
        seq.end()
    }
}

impl Serialize for ConcreteSyntax {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut seq = serializer.serialize_seq(None)?;
        match self {
            Var(id) => {
                seq.serialize_element("Var")?;
                seq.serialize_element(&id)?;
            },
            Lit(n) => {
                seq.serialize_element("Lit")?;
                seq.serialize_element(&n)?;
            },
            Pi(vc, node) => {
                seq.serialize_element("Pi")?;
                seq.serialize_element(&vc)?;
                seq.serialize_element(&node)?;
            },
            Lam(vi, node) => {
                seq.serialize_element("Lam")?;
                seq.serialize_element(&vi)?;
                seq.serialize_element(&node)?;
            },
            Ap(node, vn) => {
                seq.serialize_element("Ap")?;
                seq.serialize_element(&node)?;
                seq.serialize_element(&vn)?;
            },
            Sg(vc, node) => {
                seq.serialize_element("Sg")?;
                seq.serialize_element(&vc)?;
                seq.serialize_element(&node)?;
            },
            Type => seq.serialize_element("Type")?,
            Hole(h) => {
                seq.serialize_element("Hole")?;
                seq.serialize_element(&h)?;
            },
            BoundaryHole(on) => {
                seq.serialize_element("BoundaryHole")?;
                seq.serialize_element(&on)?;
            },
            Underscore => seq.serialize_element("Underscore")?,
            Dim => seq.serialize_element("Dim")?,
            Cof => seq.serialize_element("Cof")?,
            CofEq(node1, node2) => {
                seq.serialize_element("CofEq")?;
                seq.serialize_element(&node1)?;
                seq.serialize_element(&node2)?;
            },
            CofLe(node1, node2) => {
                seq.serialize_element("CofLe")?;
                seq.serialize_element(&node1)?;
                seq.serialize_element(&node2)?;
            },
            Join(vn) => {
                seq.serialize_element("Join")?;
                seq.serialize_element(&vn)?;
            },
            Meet(vn) => {
                seq.serialize_element("Meet")?;
                seq.serialize_element(&vn)?;
            },
            CofBoundary(node) => {
                seq.serialize_element("CofBoundary")?;
                seq.serialize_element(&node)?;
            },
            CofSplit(vnn) => {
                seq.serialize_element("CofSplit")?;
                seq.serialize_element(&vnn)?;
            },
            TopC => seq.serialize_element("TopC")?,
            BotC => seq.serialize_element("BotC")?,
            HComChk(node1, node2, node3) => {
                seq.serialize_element("HComChk")?;
                seq.serialize_element(&node1)?;
                seq.serialize_element(&node2)?;
                seq.serialize_element(&node3)?;
            },
            HFillChk(node1, node2) => {
                seq.serialize_element("HFillChk")?;
                seq.serialize_element(&node1)?;
                seq.serialize_element(&node2)?;
            },
        }
        seq.end()
    }
}
