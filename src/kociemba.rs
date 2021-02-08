use crate::rubiks_cube::{Rotation, EdgePerm, EdgeOrient, CornerOrient, CubeRepr, CornerPerm, eo_encode, co_encode, cp_encode, CubeSequenceRepr};
use crate::heuristic::*;
use std::process::exit;

/// Kociemba algorithm

pub struct KociembaSolver {
    initial: CubeSequenceRepr,
    current_solve: Vec<Rotation>,
    phase1_moves: Vec<Rotation>,
    phase2_moves: Vec<Rotation>,
}

impl KociembaSolver {
    pub fn new() -> Self {
        KociembaSolver {
            initial: CubeSequenceRepr::new(),
            current_solve: vec![],
            phase1_moves: vec![],
            phase2_moves: vec![],
        }
    }

    pub fn solve(&mut self, scrambled: &CubeSequenceRepr) {
        self.initial = scrambled.clone();
        self.solve_phase1();
    }
}

#[derive(Copy, Clone)]
struct Phase1Repr {
    eo: EdgeOrient,
    co: CornerOrient,
    ep: EdgePerm,
}

impl Phase1Repr {
    pub fn new() -> Self {
        Phase1Repr {
            eo: EdgeOrient::new(),
            co: CornerOrient::new(),
            ep: EdgePerm::new(),
        }
    }

    pub fn ok(&self) -> bool {
        !self.eo.0.contains(&true) &&
            !self.co.0.iter().any(|x| x > &0) &&
            !self.ep.0[4..8].iter().any(|x| x < &4 || x >= &8)
    }
}

impl CubeRepr for Phase1Repr {
    fn rotate(&mut self, r: Rotation) {
        self.eo.rotate(r);
        self.co.rotate(r);
        self.ep.rotate(r);
    }
}

#[derive(Copy, Clone)]
struct Phase2Repr {
    ep: EdgePerm,
    cp: CornerPerm,
}

impl Phase2Repr {
    pub fn new() -> Self {
        Phase2Repr {
            ep: EdgePerm::new(),
            cp: CornerPerm::new(),
        }
    }

    pub fn ok(&self) -> bool {
        self.ep.0.iter().enumerate().all(|(x, y)| x == *y as usize) &&
            self.cp.0.iter().enumerate().all(|(x, y)| x == *y as usize)
    }
}

impl CubeRepr for Phase2Repr {
    fn rotate(&mut self, r: Rotation) {
        self.ep.rotate(r);
        self.cp.rotate(r);
    }
}


impl KociembaSolver {
    fn h1(repr: &Phase1Repr) -> i8 {
        let h1 = PHASE1_EDGEORIENT_PT[eo_encode(&repr.eo)];
        let h2 = PHASE1_CORNERORIENT_PT[co_encode(&repr.co)];
        let h3 = PHASE1_MEDGE_PT[phase1_medge_encode_opt(&repr.ep)];
        h1.max(h2.max(h3))
    }

    fn h2(repr: &Phase2Repr) -> i8 {
        let h1 = PHASE2_UDEDGE_PT[phase2_udedge_encode(&repr.ep)];
        let h2 = PHASE2_MEDGE_PT[phase2_medge_encode(&repr.ep)];
        let h3 = PHASE2_CORNERPERM_PT[cp_encode(&repr.cp)];
        h1.max(h2.max(h3))
    }

    fn solve_phase1(&mut self) {
        let repr = Phase1Repr {
            eo: self.initial.eo,
            co: self.initial.co,
            ep: self.initial.ep,
        };
        let start_idx = KociembaSolver::h1(&repr);
        for i in start_idx..=12 {
            // println!("Phase 1 searching depth {}", i);
            self.phase1_moves.clear();
            self.search_phase1(repr.clone(), i);
        }
    }

    fn search_phase1(&mut self, repr: Phase1Repr, depth: i8) {
        if repr.ok() {
            // println!("Found phase 1 solution({}): {:?}", self.phase1_moves.len(), self.phase1_moves);
            self.solve_phase2();
            return
        }
        for r in &ALL_MOVES {
            if !self.phase1_moves.is_empty() &&
                prune_move(*self.phase1_moves.last().unwrap(), *r) {
                continue;
            }
            let mut nr = repr.clone();
            nr.rotate(*r);
            if KociembaSolver::h1(&nr) <= depth {
                self.phase1_moves.push(*r);
                self.search_phase1(nr, depth - 1);
                self.phase1_moves.pop();
            }
        }
    }

    fn solve_phase2(&mut self) {
        let repr = {
            let mut repr = Phase2Repr {
                ep: self.initial.ep,
                cp: self.initial.cp,
            };
            for r in &self.phase1_moves {
                repr.rotate(*r);
            }
            repr
        };
        let start_depth = KociembaSolver::h2(&repr);
        let max_depth = if self.current_solve.is_empty() {
            18
        } else {
            self.current_solve.len() - self.phase1_moves.len() - 2
        } as i8;
        for i in start_depth..=max_depth {
            self.phase2_moves.clear();
            if self.search_phase2(repr.clone(), i) {
                return
            }
        }
    }

    fn search_phase2(&mut self, repr: Phase2Repr, depth: i8) -> bool {
        if repr.ok() {
            // print result
            self.current_solve = self.phase1_moves.clone();
            self.current_solve.extend(&self.phase2_moves);
            println!("Found solution({}): {:?}", self.current_solve.len(), self.current_solve);
            if self.current_solve.len() <= 22 {
                exit(0);
            }
            return true;
        }
        for r in &PHASE2_MOVES {
            if !self.phase2_moves.is_empty() &&
                prune_move(*self.phase2_moves.last().unwrap(), *r) {
                continue
            }
            let mut nc = repr.clone();
            nc.rotate(*r);
            if KociembaSolver::h2(&nc) <= depth {
                self.phase2_moves.push(*r);
                if self.search_phase2(nc, depth - 1) {
                    return true;
                }
                self.phase2_moves.pop();
            }
        }
        return false;
    }
}