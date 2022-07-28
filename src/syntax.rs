use serde::{ser::SerializeSeq, *};
use slotmap::*;
use std::rc::Rc;

#[derive(Clone, Debug)]
pub enum Ident {
    Anon,
    User(Vec<String>),
    Machine(String),
}

new_key_type! {
    pub struct SyntaxRef;
}

// #[derive(Clone, Debug, Serialize)]
// pub struct Cell {
//     pub names: Vec<Ident>,
//     pub ty: Node,
// }

#[derive(Clone, Debug, Serialize)]
pub struct Hole {
    pub name: Option<String>,
    pub silent: bool,
}

#[derive(Clone, Debug)]
pub enum ConcreteSyntaxData {
    Var(Ident),
    Lit(u32),
    // Pi(Vec<Cell>, SyntaxRef),
    Lam(Vec<Ident>, SyntaxRef),
    Ap(SyntaxRef, Vec<SyntaxRef>),
    // Sg(Vec<Cell>, SyntaxRef),
    Type,
    Hole(Hole),
    Underscore,
    Dim,
    Cof,
    CofEq(SyntaxRef, SyntaxRef),
    CofLe(SyntaxRef, SyntaxRef),
    Join(Vec<SyntaxRef>),
    Meet(Vec<SyntaxRef>),
    CofBoundary(SyntaxRef),
    CofSplit(Vec<(SyntaxRef, SyntaxRef)>),
    TopC,
    BotC,
    HComChk(SyntaxRef, SyntaxRef, SyntaxRef),
    HFillChk(SyntaxRef, SyntaxRef),
}

#[derive(Debug, Serialize)]
pub struct Node {
    pub node: ConcreteSyntax,
}

#[derive(Debug)]
pub struct ConcreteSyntax(pub Rc<SlotMap<SyntaxRef, ConcreteSyntaxData>>, pub SyntaxRef);

use crate::syntax::ConcreteSyntaxData::*;
use crate::Ident::*;

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
            }
            Machine(st) => {
                seq.serialize_element("Machine")?;
                seq.serialize_element(st)?;
            }
        }
        seq.end()
    }
}

impl Node {
    fn new(map: &Rc<SlotMap<SyntaxRef, ConcreteSyntaxData>>, next: &SyntaxRef) -> Node {
        Node {
            node: ConcreteSyntax(map.clone(), *next)
        }
    }
}

impl Serialize for ConcreteSyntax {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let ConcreteSyntax(map, current) = self;
        let mut seq = serializer.serialize_seq(None)?;
        match &map[*current] {
            Var(id) => {
                seq.serialize_element("Var")?;
                seq.serialize_element(&id)?;
            }
            Lit(n) => {
                seq.serialize_element("Lit")?;
                seq.serialize_element(&n)?;
            }
            Lam(vi, sref) => {
                seq.serialize_element("Lam")?;
                seq.serialize_element(&vi)?;
                seq.serialize_element(&Node::new(map, sref))?;
            }
            Ap(sref, vsref) => {
                seq.serialize_element("Ap")?;
                seq.serialize_element(&Node::new(map, sref))?;
                let vnode: Vec<Node> =
                    vsref.iter().map(|n| Node::new(map, n)).collect();
                seq.serialize_element(&vnode)?;
            }
            Type => seq.serialize_element("Type")?,
            Hole(h) => {
                seq.serialize_element("Hole")?;
                seq.serialize_element(&h)?;
            }
            Underscore => seq.serialize_element("Underscore")?,
            Dim => seq.serialize_element("Dim")?,
            Cof => seq.serialize_element("Cof")?,
            CofEq(sref1, sref2) => {
                seq.serialize_element("CofEq")?;
                seq.serialize_element(&Node::new(map, sref1))?;
                seq.serialize_element(&Node::new(map, sref2))?;
            }
            CofLe(sref1, sref2) => {
                seq.serialize_element("CofLe")?;
                seq.serialize_element(&Node::new(map, sref1))?;
                seq.serialize_element(&Node::new(map, sref2))?;
            }
            Join(vsref) => {
                seq.serialize_element("Join")?;
                let vnode: Vec<Node> =
                    vsref.iter().map(|n| Node::new(map, n)).collect();
                seq.serialize_element(&vnode)?;
            }
            Meet(vsref) => {
                seq.serialize_element("Meet")?;
                let vnode: Vec<Node> =
                    vsref.iter().map(|n| Node::new(map, n)).collect();
                seq.serialize_element(&vnode)?;
            }
            CofBoundary(sref) => {
                seq.serialize_element("CofBoundary")?;
                seq.serialize_element(&Node::new(map, sref))?;
            }
            CofSplit(vsrefp) => {
                seq.serialize_element("CofSplit")?;
                let vnodep: Vec<(Node, Node)> = vsrefp
                    .iter()
                    .map(|(n1, n2)| {
                        (
                            Node::new(map, n1),
                            Node::new(map, n2),
                        )
                    })
                    .collect();
                seq.serialize_element(&vnodep)?;
            }
            TopC => seq.serialize_element("TopC")?,
            BotC => seq.serialize_element("BotC")?,
            HComChk(sref1, sref2, sref3) => {
                seq.serialize_element("HComChk")?;
                seq.serialize_element(&Node::new(map, sref1))?;
                seq.serialize_element(&Node::new(map, sref2))?;
                seq.serialize_element(&Node::new(map, sref3))?;
            }
            HFillChk(sref1, sref2) => {
                seq.serialize_element("HFillChk")?;
                seq.serialize_element(&Node::new(map, sref1))?;
                seq.serialize_element(&Node::new(map, sref2))?;
            }
        }
        seq.end()
    }
}
