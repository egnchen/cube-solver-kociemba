use cube_solver::heuristic::{ALL_MOVES, phase1_medge_encode, PruneTable};
use cube_solver::rubiks_cube::*;
use cube_solver::kociemba::KociembaSolver;

fn main() {
    let moves = [
        Rotation::F2, Rotation::D, Rotation::L, Rotation::U2, Rotation::B2, Rotation::Lr, Rotation::B2, Rotation::L2, Rotation::R, Rotation::F2, Rotation::Rr, Rotation::D2, Rotation::R2, Rotation::F, Rotation::D, Rotation::U2, Rotation::B, Rotation::D, Rotation::Rr, Rotation::U2
    ];
    let cube = CubeSequenceRepr::from(&moves);
    let mut vis = ColoredCube::new();
    cube.visualize(&mut vis);
    println!("{}", vis);

    let mut solver = KociembaSolver::new();
    solver.solve(&cube);
}