use std::fmt::{Display, Formatter, Result};
use std::ops::{Index, IndexMut};

use crate::num_traits::{FromPrimitive, ToPrimitive};
use crate::util::encode_perm;

#[derive(FromPrimitive, ToPrimitive)]
#[derive(Copy, Clone)]
#[derive(PartialOrd, PartialEq)]
#[derive(Debug)]
pub enum Rotation {
    U = 0,
    D,
    R,
    L,
    F,
    B,
    Ur,
    Dr,
    Rr,
    Lr,
    Fr,
    Br,
    U2,
    D2,
    R2,
    L2,
    F2,
    B2,
}

impl Rotation {
    pub fn reverse(&self) -> Rotation {
        let num = self.to_u8().unwrap();
        num_traits::FromPrimitive::from_u8(match num {
            0..=5 => num + 6,
            6..=11 => num - 6,
            _ => num,
        }).unwrap()
    }

    pub fn normal(&self) -> Rotation {
        num_traits::FromPrimitive::from_u8(self.to_u8().unwrap() % 6).unwrap()
    }

    pub fn is_cw(&self) -> bool {
        self.lt(&Rotation::Ur)
    }

    pub fn is_ccw(&self) -> bool {
        self.ge(&Rotation::Ur) && self.le(&Rotation::U2)
    }

    pub fn is_180(&self) -> bool {
        self.ge(&Rotation::U2)
    }
}

#[derive(Copy, Clone)]
#[derive(FromPrimitive, ToPrimitive)]
pub enum Face {
    U = 0,
    D,
    R,
    L,
    F,
    B,
}

pub trait CubeRepr {
    fn rotate(&mut self, r: Rotation);
}

pub struct ArrayStruct<T>([T; 12]);

#[derive(Debug, Copy, Clone)]
pub struct EdgePerm(pub [u8; 12]);

#[derive(Debug, Copy, Clone)]
pub struct EdgeOrient(pub [bool; 12]);

#[derive(Debug, Copy, Clone)]
pub struct CornerPerm(pub [u8; 8]);

#[derive(Debug, Copy, Clone)]
pub struct CornerOrient(pub [u8; 8]);

const CORNER_GROUP: [[usize; 4]; 6] = [
    [0, 1, 2, 3],
    [7, 6, 5, 4],
    [2, 1, 5, 6],
    [0, 3, 7, 4],
    [3, 2, 6, 7],
    [1, 0, 4, 5],
];

const EDGE_GROUP: [[usize; 4]; 6] = [
    [0, 1, 2, 3],
    [11, 10, 9, 8],
    [1, 5, 9, 6],
    [3, 7, 11, 4],
    [2, 6, 10, 7],
    [0, 4, 8, 5],
];

fn rotate_seq<T: Sized + Copy>(seq: &mut [T], r: Rotation, group: &[usize; 4]) {
    let dir = r.is_cw();
    if r.is_180() {
        rotate_cw(seq, group);
        rotate_cw(seq, group);
    } else if dir {
        rotate_cw(seq, group);
    } else {
        rotate_ccw(seq, group);
    }
}

fn rotate_cw<T: Sized + Copy>(seq: &mut [T], group: &[usize; 4]) {
    let tmp = seq[group[0]];
    seq[group[0]] = seq[group[3]];
    seq[group[3]] = seq[group[2]];
    seq[group[2]] = seq[group[1]];
    seq[group[1]] = tmp;
}

fn rotate_ccw<T: Sized + Copy>(seq: &mut [T], group: &[usize; 4]) {
    let tmp = seq[group[0]];
    seq[group[0]] = seq[group[1]];
    seq[group[1]] = seq[group[2]];
    seq[group[2]] = seq[group[3]];
    seq[group[3]] = tmp;
}

impl EdgePerm {
    pub fn new() -> Self {
        EdgePerm {
            0: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
        }
    }
}

impl EdgeOrient {
    pub fn new() -> Self {
        EdgeOrient {
            0: [false; 12]
        }
    }
}

impl CornerPerm {
    pub fn new() -> Self {
        CornerPerm {
            0: [0, 1, 2, 3, 4, 5, 6, 7],
        }
    }
}

impl CornerOrient {
    pub fn new() -> Self {
        CornerOrient {
            0: [0; 8]
        }
    }
}

impl CubeRepr for EdgePerm {
    fn rotate(&mut self, r: Rotation) {
        rotate_seq(&mut self.0, r, &EDGE_GROUP[r.normal().to_usize().unwrap()])
    }
}

impl CubeRepr for EdgeOrient {
    fn rotate(&mut self, r: Rotation) {
        let norm = r.normal();
        rotate_seq(&mut self.0, r, &EDGE_GROUP[norm.to_usize().unwrap()]);
        if !r.is_180() {
            if norm == Rotation::R || norm == Rotation::L {
                for &i in EDGE_GROUP[norm.to_usize().unwrap()].iter() {
                    self.0[i] = !self.0[i];
                }
            }
        }
    }
}

impl CubeRepr for CornerPerm {
    fn rotate(&mut self, r: Rotation) {
        rotate_seq(&mut self.0, r, &CORNER_GROUP[r.normal().to_usize().unwrap()]);
    }
}

impl CubeRepr for CornerOrient {
    fn rotate(&mut self, r: Rotation) {
        let norm = r.normal();
        rotate_seq(&mut self.0, r, &CORNER_GROUP[norm.to_usize().unwrap()]);
        if !r.is_180() {
            if norm != Rotation::U && norm != Rotation::D {
                for i in 0..4 {
                    let idx = CORNER_GROUP[norm.to_usize().unwrap()][i];
                    // 1, 2, 1, 2
                    self.0[idx] = (self.0[idx] + 1 + (i as u8 % 2)) % 3;
                }
            }
        }
    }
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub struct CubeSequenceRepr {
    pub ep: EdgePerm,
    pub cp: CornerPerm,
    pub eo: EdgeOrient,
    pub co: CornerOrient,
}

impl CubeRepr for CubeSequenceRepr {
    fn rotate(&mut self, r: Rotation) {
        self.ep.rotate(r);
        self.cp.rotate(r);
        self.eo.rotate(r);
        self.co.rotate(r);
    }
}

impl CubeSequenceRepr {
    pub fn new() -> Self {
        CubeSequenceRepr {
            ep: EdgePerm::new(),
            cp: CornerPerm::new(),
            eo: EdgeOrient::new(),
            co: CornerOrient::new(),
        }
    }

    pub fn from(moves: &[Rotation]) -> Self {
        let mut repr = CubeSequenceRepr::new();
        for r in moves {
            repr.rotate(*r);
        }
        repr
    }

    pub fn visualize(&self, buf: &mut ColoredCube) {
        const CORNER_MAP: [[Face; 3]; 8] = [
            [Face::U, Face::L, Face::B], [Face::U, Face::B, Face::R], [Face::U, Face::R, Face::F], [Face::U, Face::F, Face::L],
            [Face::D, Face::B, Face::L], [Face::D, Face::R, Face::B], [Face::D, Face::F, Face::R], [Face::D, Face::L, Face::F]
        ];
        const EDGE_MAP: [[Face; 2]; 12] = [
            [Face::U, Face::B], [Face::U, Face::R], [Face::U, Face::F], [Face::U, Face::L],
            [Face::L, Face::B], [Face::R, Face::B], [Face::R, Face::F], [Face::L, Face::F],
            [Face::D, Face::B], [Face::D, Face::R], [Face::D, Face::F], [Face::D, Face::L]
        ];
        const CORNERS: [[usize; 2]; 8] = [
            [0, 0], [0, 2], [2, 2], [2, 0],
            [2, 0], [2, 2], [0, 2], [0, 0]
        ];
        const EDGES: [[usize; 2]; 12] = [
            [0, 1], [1, 2], [2, 1], [1, 0],
            [1, 0], [1, 2], [1, 0], [1, 2],
            [2, 1], [1, 2], [0, 1], [1, 0]
        ];
        // color all corners
        for i in 0..8 {
            buf[CORNER_MAP[i][0]][CORNERS[i][0]][CORNERS[i][1]] = CORNER_MAP[self.cp.0[i] as usize][self.co.0[i] as usize];
        }
        for i in 0..4 {
            buf[CORNER_MAP[i][1]][0][0] = CORNER_MAP[self.cp.0[i] as usize][((self.co.0[i] + 1) % 3) as usize];
            buf[CORNER_MAP[i][2]][0][2] = CORNER_MAP[self.cp.0[i] as usize][((self.co.0[i] + 2) % 3) as usize];
        }
        for i in 4..8 {
            buf[CORNER_MAP[i][1]][2][2] = CORNER_MAP[self.cp.0[i] as usize][((self.co.0[i] + 1) % 3) as usize];
            buf[CORNER_MAP[i][2]][2][0] = CORNER_MAP[self.cp.0[i] as usize][((self.co.0[i] + 2) % 3) as usize];
        }
        // color all edges
        for i in 0..12 {
            buf[EDGE_MAP[i][0]][EDGES[i][0] as usize][EDGES[i][1] as usize] = EDGE_MAP[self.ep.0[i] as usize][self.eo.0[i] as usize];
            if i < 4 {
                buf[EDGE_MAP[i][1]][0][1] = EDGE_MAP[self.ep.0[i] as usize][!self.eo.0[i] as usize];
            } else if i < 8 {
                // 1,2 1,0 1,2 1,0
                buf[EDGE_MAP[i][1]][1][2 - 2 * (i % 2)] = EDGE_MAP[self.ep.0[i] as usize][!self.eo.0[i] as usize];
            } else {
                buf[EDGE_MAP[i][1]][2][1] = EDGE_MAP[self.ep.0[i] as usize][!self.eo.0[i] as usize];
            }
        }
        // color all middles
        for f in [Face::U, Face::D, Face::L, Face::R, Face::F, Face::B].iter() {
            buf[f.to_usize().unwrap()][1][1] = *f;
        }
    }
}

// visualizer
pub struct ColoredCube {
    pub dat: [[[Face; 3]; 3]; 6]
}

impl Index<usize> for ColoredCube {
    type Output = [[Face; 3]; 3];

    fn index(&self, index: usize) -> &Self::Output {
        &self.dat[index]
    }
}

impl IndexMut<usize> for ColoredCube {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.dat[index]
    }
}

impl Index<Face> for ColoredCube {
    type Output = [[Face; 3]; 3];

    fn index(&self, index: Face) -> &Self::Output {
        &self[index.to_usize().unwrap()]
    }
}

impl IndexMut<Face> for ColoredCube {
    fn index_mut(&mut self, index: Face) -> &mut Self::Output {
        self.index_mut(index.to_usize().unwrap())
    }
}

const COLOR_MAP: [&'static str; 6] = [
    "W", "Y", "R", "O", "G", "B"
];

const SEP: &'static str = " ";

impl Display for ColoredCube {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for i in 0..3 {
            for _ in 0..3 { write!(f, "{}", SEP)?; }
            for j in 0..3 { write!(f, "{}", COLOR_MAP[self.dat[Face::U.to_usize().unwrap()][i][j].to_usize().unwrap()])?; }
            write!(f, "\n")?;
        }
        for j in 0..3 {
            for i in [Face::L, Face::F, Face::R, Face::B].iter() {
                for k in 0..3 {
                    write!(f, "{}", COLOR_MAP[self.dat[i.to_usize().unwrap()][j][k].to_usize().unwrap()])?;
                }
            }
            write!(f, "\n")?;
        }
        for i in 0..3 {
            for _ in 0..3 { write!(f, "{}", SEP)?; }
            for j in 0..3 { write!(f, "{}", COLOR_MAP[self.dat[Face::D.to_usize().unwrap()][i][j].to_usize().unwrap()])?; }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl ColoredCube {
    pub fn new() -> Self {
        ColoredCube {
            dat: [[[Face::U; 3]; 3]; 6]
        }
    }
}

// various encoders
pub fn ep_encode(repr: &EdgePerm) -> usize {
    encode_perm(&repr.0)
}

pub fn eo_encode(repr: &EdgeOrient) -> usize {
    let mut res: usize = 0;
    for i in 0..11 {
        res = res * 2 + (repr.0[i] as usize);
    }
    res
}

pub fn cp_encode(repr: &CornerPerm) -> usize {
    encode_perm(&repr.0)
}

pub fn co_encode(repr: &CornerOrient) -> usize {
    let mut res = 0;
    for i in 0..7 {
        res = res * 3 + (repr.0[i] as usize);
    }
    res
}