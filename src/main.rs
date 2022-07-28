use slotmap::SlotMap;
use std::rc::Rc;
use syntax::{ConcreteSyntax, ConcreteSyntaxData, Hole, Ident, SyntaxRef, Node};

mod camera;
mod cube;
mod label;
mod linalg;
mod messages;
mod render;
mod server;
mod syntax;
mod system;
mod termbuilder;
mod vertex;

fn main() {
    // render::render();

    let i_ident = Ident::User(vec!["i".to_string()]);
    let j_ident = Ident::User(vec!["j".to_string()]);

    let mut syn = SlotMap::with_key();

    let dim0: SyntaxRef = syn.insert(ConcreteSyntaxData::Lit(0));
    let dim1: SyntaxRef = syn.insert(ConcreteSyntaxData::Lit(1));
    let i: SyntaxRef = syn.insert(ConcreteSyntaxData::Var(i_ident));
    let j: SyntaxRef = syn.insert(ConcreteSyntaxData::Var(j_ident.clone()));
    let hole: SyntaxRef = syn.insert(ConcreteSyntaxData::Hole(Hole {
        name: None,
        silent: false,
    }));

    let j_eq_0: SyntaxRef = syn.insert(ConcreteSyntaxData::CofEq(j, dim0));
    let i_eq_0: SyntaxRef = syn.insert(ConcreteSyntaxData::CofEq(i, dim0));
    let i_eq_1: SyntaxRef = syn.insert(ConcreteSyntaxData::CofEq(i, dim1));

    let split: SyntaxRef = syn.insert(ConcreteSyntaxData::CofSplit(vec![
        (j_eq_0, hole),
        (i_eq_0, hole),
        (i_eq_1, hole),
    ]));
    let lam: SyntaxRef = syn.insert(ConcreteSyntaxData::Lam(vec![j_ident], split));
    let hcom: SyntaxRef = syn.insert(ConcreteSyntaxData::HComChk(dim0, dim1, lam));

    println!(
        "{:?}",
        serde_json::to_string(&Node {
            node: ConcreteSyntax(Rc::new(syn), hcom)
        })
    );
}
