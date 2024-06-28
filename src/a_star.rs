use bevy::prelude::*;
use pathfinding::prelude::astar;
use std::rc::Rc;

// #[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
// struct Pos(i32, i32);
//
// impl Pos {
//     fn successors(&self) -> Vec<Pos> {
//         let &Pos(x, y) = self;
//         vec![Pos(x+1,y+2), Pos(x+1,y-2), Pos(x-1,y+2), Pos(x-1,y-2),
//              Pos(x+2,y+1), Pos(x+2,y-1), Pos(x-2,y+1), Pos(x-2,y-1)]
//     }
// }
//
// static GOAL: Pos = Pos(4, 6);
// let result = bfs(&Pos(1, 1), |p| p.successors(), |p| *p == GOAL);

struct AStarNode {
    position: IVec3,
    tiles: Rc<Vec<IVec3>>,
}
impl AStarNode {
    fn successors(&self) -> Vec<AStarNode> {
        let mut successors = vec![];
        for j in -1..1 {
            for i in -1..1 {
                if i == 0 && j == 0 {
                    continue;
                }

                if self.tiles.contains(&(self.position + IVec3::new(i, 0, j))) {
                    successors.push(AStarNode {
                        position: self.position + IVec3::new(i, 0, j),
                        tiles: self.tiles.clone(),
                    });
                }
            }
        }

        successors
    }
}

pub struct AStar;
impl AStar {
    pub fn new(tiles: Vec<Vec3>) -> Self {
        let tiles: Vec<IVec3> = tiles
            .iter()
            .map(|p| {
                IVec3::new(
                    (1000. * p.x.round()) as i32,
                    (1000. * p.y.round()) as i32,
                    (1000. * p.z.round()) as i32,
                )
            })
            .collect();
        AStar
    }

    pub fn path(&self, origin: Vec3, target: Vec3) -> Vec<Vec3> {
        vec![origin, target]
    }
}
