use syntax::{ConcreteSyntax, Hole, Ident, Node};

mod camera;
mod cube;
mod label;
mod linalg;
mod messages;
mod render;
mod server;
mod syntax;
mod system;
mod vertex;

use server::Server;

fn main() {
    // render::render();

    let server = Server::init(3001);
    let i = Ident::User(vec!["i".to_string()]);
    let j = Ident::User(vec!["j".to_string()]);
    let hcom = Node::new(ConcreteSyntax::HComChk(
        Node::new(ConcreteSyntax::Lit(0)),
        Node::new(ConcreteSyntax::Lit(1)),
        Node::new(ConcreteSyntax::Lam(
            vec![j.clone()],
            Node::new(ConcreteSyntax::CofSplit(vec![
                (
                    Node::new(ConcreteSyntax::CofEq(
                        Node::new(ConcreteSyntax::Var(j)),
                        Node::new(ConcreteSyntax::Lit(0)),
                    )),
                    Node::new(ConcreteSyntax::Hole(Hole {
                        name: None,
                        silent: false,
                    })),
                ),
                (
                    Node::new(ConcreteSyntax::CofEq(
                        Node::new(ConcreteSyntax::Var(i.clone())),
                        Node::new(ConcreteSyntax::Lit(0)),
                    )),
                    Node::new(ConcreteSyntax::Hole(Hole {
                        name: None,
                        silent: false,
                    })),
                ),
                (
                    Node::new(ConcreteSyntax::CofEq(
                        Node::new(ConcreteSyntax::Var(i)),
                        Node::new(ConcreteSyntax::Lit(1)),
                    )),
                    Node::new(ConcreteSyntax::Hole(Hole {
                        name: None,
                        silent: false,
                    })),
                ),
            ])),
        )),
    ));

    println!("{:?}", serde_json::to_string(&hcom));
}
