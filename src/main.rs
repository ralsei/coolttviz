use crate::syntax::*;
use std::rc::Rc;

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

    let i = Ident::User(vec!["i".to_string()]);
    let j = Ident::User(vec!["j".to_string()]);
    let hcom = ConcreteSyntax::HComChk(
        SyntaxRec::new(ConcreteSyntax::Lit(0)),
        SyntaxRec::new(ConcreteSyntax::Lit(1)),
        SyntaxRec::new(ConcreteSyntax::Lam(
            vec![j.clone()],
            SyntaxRec::new(ConcreteSyntax::CofSplit(vec![
                (
                    SyntaxRec::new(ConcreteSyntax::CofEq(
                        SyntaxRec::new(ConcreteSyntax::Var(j)),
                        SyntaxRec::new(ConcreteSyntax::Lit(0)),
                    )),
                    SyntaxRec::new(ConcreteSyntax::Hole(Hole {
                        name: None,
                        silent: false,
                    })),
                ),
                (
                    SyntaxRec::new(ConcreteSyntax::CofEq(
                        SyntaxRec::new(ConcreteSyntax::Var(i.clone())),
                        SyntaxRec::new(ConcreteSyntax::Lit(0)),
                    )),
                    SyntaxRec::new(ConcreteSyntax::Hole(Hole {
                        name: None,
                        silent: false,
                    })),
                ),
                (
                    SyntaxRec::new(ConcreteSyntax::CofEq(
                        SyntaxRec::new(ConcreteSyntax::Var(i)),
                        SyntaxRec::new(ConcreteSyntax::Lit(1)),
                    )),
                    SyntaxRec::new(ConcreteSyntax::Hole(Hole {
                        name: None,
                        silent: false,
                    })),
                ),
            ])),
        )),
    );

    let mut allocator = termbuilder::Term::new();
    let top = allocator.plug(hcom);

    println!(
        "{:?}",
        serde_json::to_string(&Node {
            node: SerializableSyntax(Rc::new(allocator.map), top)
        })
    );
}
