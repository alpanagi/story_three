use std::rc::Rc;

use bevy::prelude::*;
use pathfinding::prelude::astar;

#[derive(Eq, Hash, Clone, Debug)]
struct AStarNode {
    pub position: IVec3,
    tiles: Rc<Vec<IVec3>>,
}

impl PartialEq for AStarNode {
    fn eq(&self, other: &Self) -> bool {
        self.position == other.position
    }
}

impl AStarNode {
    fn new(position: IVec3, tiles: Rc<Vec<IVec3>>) -> Self {
        AStarNode { position, tiles }
    }

    fn distance(&self, other: &AStarNode) -> u32 {
        self.position.distance_squared(other.position) as u32
    }

    fn successors(&self) -> Vec<(AStarNode, u32)> {
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
            .into_iter()
            .map(|p| (p.clone(), p.distance(self)))
            .collect()
    }
}

pub struct AStar {
    tiles: Rc<Vec<IVec3>>,
}
impl AStar {
    pub fn new(tiles: Vec<Vec3>) -> Self {
        let tiles: Vec<IVec3> = tiles.iter().map(|p| to_ivec3(p)).collect();
        AStar {
            tiles: Rc::new(tiles),
        }
    }

    pub fn path(&self, origin: Vec3, target: Vec3) -> Option<Vec<Vec3>> {
        let origin = &AStarNode::new(to_ivec3(&origin), self.tiles.clone());
        let target = AStarNode::new(to_ivec3(&target), self.tiles.clone());
        let path = astar(
            origin,
            |p| p.successors(),
            |p| p.distance(&target),
            |p| *p == target,
        );
        println!("{:?}", path);
        path.map(|x| {
            x.0.iter()
                .map(|x| x.position.as_vec3())
                .collect::<Vec<Vec3>>()
        })
    }
}

fn to_ivec3(val: &Vec3) -> IVec3 {
    IVec3::new(
        (1000. * val.x.round()) as i32,
        (1000. * val.y.round()) as i32,
        (1000. * val.z.round()) as i32,
    )
}
