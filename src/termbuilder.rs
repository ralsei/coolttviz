use std::collections::VecDeque;

use crate::syntax::{ConcreteSyntax, ConcreteSyntax::*, Hole, SyntaxRec, SyntaxRef};
use slotmap::SlotMap;

pub struct Term {
    pub map: SlotMap<SyntaxRef, ConcreteSyntax<SyntaxRef>>,
    holes: VecDeque<SyntaxRef>,
}

impl Term {
    pub fn new() -> Term {
        let mut map = SlotMap::with_key();
        let hole: SyntaxRef = map.insert(ConcreteSyntax::Hole(Hole {
            name: None,
            silent: false,
        }));

        Term {
            map,
            holes: VecDeque::from([hole]),
        }
    }

    pub fn plug(&mut self, filler: ConcreteSyntax<SyntaxRec>) -> SyntaxRef {
        let hole = self.holes.pop_front().expect("No holes in term!");
        self.map[hole] = self.derecurse(filler);
        hole
    }

    fn derecurse(&mut self, filler: ConcreteSyntax<SyntaxRec>) -> ConcreteSyntax<SyntaxRef> {
        match filler {
            Var(id) => Var(id),
            Lit(n) => Lit(n),
            Lam(vi, rec) => {
                let cs = self.derecurse(*rec.value);
                let sref: SyntaxRef = self.map.insert(cs);
                Lam(vi, sref)
            }
            Ap(rec, vrec) => {
                let cs = self.derecurse(*rec.value);
                let sref: SyntaxRef = self.map.insert(cs);
                let mut vsref: Vec<SyntaxRef> = Vec::with_capacity(vrec.len());
                for rec in vrec {
                    let cs = self.derecurse(*rec.value);
                    vsref.push(self.map.insert(cs))
                }
                Ap(sref, vsref)
            }
            Type => Type,
            Hole(h) => Hole(h),
            Underscore => Underscore,
            Dim => Dim,
            Cof => Cof,
            CofEq(rec1, rec2) => {
                let cs1 = self.derecurse(*rec1.value);
                let sref1: SyntaxRef = self.map.insert(cs1);
                let cs2 = self.derecurse(*rec2.value);
                let sref2: SyntaxRef = self.map.insert(cs2);
                CofEq(sref1, sref2)
            }
            CofLe(rec1, rec2) => {
                let cs1 = self.derecurse(*rec1.value);
                let sref1: SyntaxRef = self.map.insert(cs1);
                let cs2 = self.derecurse(*rec2.value);
                let sref2: SyntaxRef = self.map.insert(cs2);
                CofLe(sref1, sref2)
            }
            Join(vrec) => {
                let mut vsref: Vec<SyntaxRef> = Vec::with_capacity(vrec.len());
                for rec in vrec {
                    let cs = self.derecurse(*rec.value);
                    vsref.push(self.map.insert(cs));
                }
                Join(vsref)
            }
            Meet(vrec) => {
                let mut vsref: Vec<SyntaxRef> = Vec::with_capacity(vrec.len());
                for rec in vrec {
                    let cs = self.derecurse(*rec.value);
                    vsref.push(self.map.insert(cs));
                }
                Meet(vsref)
            }
            CofSplit(vrecp) => {
                let mut vsrefp: Vec<(SyntaxRef, SyntaxRef)> = Vec::with_capacity(vrecp.len());
                for (rec1, rec2) in vrecp {
                    let cs1 = self.derecurse(*rec1.value);
                    let cs2 = self.derecurse(*rec2.value);
                    vsrefp.push((self.map.insert(cs1), self.map.insert(cs2)));
                }
                CofSplit(vsrefp)
            }
            TopC => TopC,
            BotC => BotC,
            HComChk(rec1, rec2, rec3) => {
                let cs1 = self.derecurse(*rec1.value);
                let sref1: SyntaxRef = self.map.insert(cs1);
                let cs2 = self.derecurse(*rec2.value);
                let sref2: SyntaxRef = self.map.insert(cs2);
                let cs3 = self.derecurse(*rec3.value);
                let sref3: SyntaxRef = self.map.insert(cs3);
                HComChk(sref1, sref2, sref3)
            }
            HFillChk(rec1, rec2) => {
                let cs1 = self.derecurse(*rec1.value);
                let sref1: SyntaxRef = self.map.insert(cs1);
                let cs2 = self.derecurse(*rec2.value);
                let sref2: SyntaxRef = self.map.insert(cs2);
                HFillChk(sref1, sref2)
            }
        }
    }
}
