use crate::rubiks_cube::*;
use crate::num_traits::ToPrimitive;
use std::collections::VecDeque;
use std::marker::PhantomData;
use crate::util::{encode_comb, encode_perm, comb};
use std::ops::Index;

/// Building heuristic tables

pub const ALL_MOVES: [Rotation; 18] = [
    Rotation::U, Rotation::D, Rotation::R, Rotation::L, Rotation::F, Rotation::B,
    Rotation::Ur, Rotation::Dr, Rotation::Rr, Rotation::Lr, Rotation::Fr, Rotation::Br,
    Rotation::U2, Rotation::D2, Rotation::R2, Rotation::L2, Rotation::F2, Rotation::B2,
];

pub const PHASE2_MOVES: [Rotation; 10] = [
    Rotation::U, Rotation::Ur, Rotation::U2,
    Rotation::D, Rotation::Dr, Rotation::D2,
    Rotation::R2, Rotation::L2, Rotation::F2, Rotation::B2
];

pub const PHASE2_MEDGE_MOVES: [Rotation; 4] = [
    Rotation::R2, Rotation::L2, Rotation::F2, Rotation::B2
];

pub struct PruneTable<T: CubeRepr> {
    table: Vec<i8>,
    _phantom: PhantomData<T>,
}

// Node data type used in PruneTable
struct PruneNode<T: Sized> {
    state: T,
    depth: i8,
    rot: Rotation,
}

// standard pruning
#[inline(always)]
pub fn prune_move(prev: Rotation, cur: Rotation) -> bool {
    let np = prev.normal().to_u8().unwrap();
    let nn = cur.normal().to_u8().unwrap();
    np == nn || (np / 2 == nn / 2 && np > nn)
}

impl<T: CubeRepr + Copy> PruneTable<T> {
    pub fn new(sz: usize) -> Self {
        PruneTable {
            table: vec![-1; sz],
            _phantom: Default::default(),
        }
    }

    pub fn init<F>(&mut self, encoder: F, initial_states: &[T], valid_moves: &[Rotation])
        where F: Fn(&T) -> usize {
        let mut q = VecDeque::new();
        for s in initial_states.iter() {
            self.table[encoder(&s)] = 0;
            for r in valid_moves.iter() {
                let mut state = s.clone();
                state.rotate(*r);
                q.push_back(PruneNode {
                    state,
                    depth: 1,
                    rot: *r,
                });
            }
        }
        while !q.is_empty() {
            let PruneNode { state, depth, rot } = q.pop_front().unwrap();
            let idx = encoder(&state);
            let prev = self.table[idx];
            if prev == -1 || prev > depth {
                self.table[idx] = depth;
                for r in valid_moves.iter() {
                    if !prune_move(rot, *r) {
                        let mut ns = state.clone();
                        ns.rotate(*r);
                        q.push_back(PruneNode {
                            state: ns,
                            depth: depth + 1,
                            rot: *r,
                        })
                    }
                }
            }
        }
    }
}

impl<T: CubeRepr + Copy> Index<usize> for PruneTable<T> {
    type Output = i8;

    fn index(&self, index: usize) -> &Self::Output {
        &self.table[index]
    }
}

// some special encoders
pub fn phase1_medge_encode(repr: &EdgePerm) -> usize {
    let mut buf = [0; 4];
    let mut j = 0;
    for i in 0..12 {
        if repr.0[i] >= 4 && repr.0[i] < 8 {
            buf[j] = i;
            j += 1;
        }
    }
    assert_eq!(j, 4);
    encode_comb(&buf, 12)
}

pub fn phase1_medge_encode_opt(repr: &EdgePerm) -> usize {
    let mut k = 4;
    let mut res = 0;
    for i in (0..12).rev() {
        if repr.0[i] >= 4 && repr.0[i] < 8 {
            k -= 1;
            if k == 0 {
                break
            }
        } else {
            res += comb(i, k - 1);
        }
    }
    res
}

pub fn phase2_medge_encode(repr: &EdgePerm) -> usize {
    let mut buf = [0u8; 4];
    for i in 4..8 {
        buf[i - 4] = repr.0[i];
    }
    encode_perm(&buf)
}

pub fn phase2_udedge_encode(repr: &EdgePerm) -> usize {
    let mut buf = [0u8; 8];
    for i in 0..4 {
        let v = repr.0[i];
        buf[i] = if v < 4 { v } else { v - 4 };
    }
    for i in 8..12 {
        let v = repr.0[i];
        buf[i - 4] = if v < 4 { v } else { v - 4 };
    }
    encode_perm(&buf)
}

lazy_static! {
    pub static ref PHASE1_EDGEORIENT_PT: PruneTable<EdgeOrient> = {
        let mut pt = PruneTable::new(2048);
        pt.init(crate::rubiks_cube::eo_encode, &[EdgeOrient::new()], &ALL_MOVES);
        pt
    };
    pub static ref PHASE1_CORNERORIENT_PT: PruneTable<CornerOrient> = {
        let mut pt = PruneTable::new(2187);
        pt.init(crate::rubiks_cube::co_encode, &[CornerOrient::new()], &ALL_MOVES);
        pt
    };
    pub static ref PHASE1_MEDGE_PT: PruneTable<EdgePerm> = {
        let mut pt = PruneTable::new(495);
        pt.init(phase1_medge_encode_opt, &[EdgePerm::new()], &ALL_MOVES);
        pt
    };
    pub static ref PHASE2_UDEDGE_PT: PruneTable<EdgePerm> = {
        let mut pt = PruneTable::new(40320);
        pt.init(phase2_udedge_encode, &[EdgePerm::new()], &PHASE2_MOVES);
        pt
    };
    pub static ref PHASE2_MEDGE_PT: PruneTable<EdgePerm> = {
        let mut pt = PruneTable::new(24);
        pt.init(phase2_medge_encode, &[EdgePerm::new()], &PHASE2_MEDGE_MOVES);
        pt
    };
    pub static ref PHASE2_CORNERPERM_PT: PruneTable<CornerPerm> = {
        let mut pt = PruneTable::new(40320);
        pt.init(crate::rubiks_cube::cp_encode, &[CornerPerm::new()], &PHASE2_MOVES);
        pt
    };
}