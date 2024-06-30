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
        self.position.as_vec3().distance(other.position.as_vec3()) as u32
    }

    fn successors(&self) -> Vec<(AStarNode, u32)> {
        let mut successors = vec![];
        let straight_offsets = vec![
            (IVec3::new(-1, 0, 0), 100),
            (IVec3::new(0, 0, -1), 100),
            (IVec3::new(0, 0, 1), 100),
            (IVec3::new(1, 0, 0), 100),
        ];
        let diagonal_offsets = vec![
            (IVec3::new(-1, 0, -1), 141),
            (IVec3::new(-1, 0, 1), 141),
            (IVec3::new(1, 0, -1), 141),
            (IVec3::new(1, 0, 1), 141),
        ];

        for (offset, dist) in straight_offsets {
            let tile = self.position + offset * 170;
            if self.tiles.contains(&tile) {
                successors.push((
                    AStarNode {
                        position: tile,
                        tiles: self.tiles.clone(),
                    },
                    dist,
                ));
            }
        }

        for (offset, dist) in diagonal_offsets {
            let tile = self.position + offset * 170;
            let wall_obstacle_a = self.position + IVec3::new(offset.x, 0, 0) * 170;
            let wall_obstacle_b = self.position + IVec3::new(0, 0, offset.z) * 170;
            if self.tiles.contains(&tile)
                && self.tiles.contains(&wall_obstacle_a)
                && self.tiles.contains(&wall_obstacle_b)
            {
                successors.push((
                    AStarNode {
                        position: tile,
                        tiles: self.tiles.clone(),
                    },
                    dist,
                ));
            }
        }

        successors
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
        path.map(|x| {
            x.0.iter()
                .map(|x| x.position.as_vec3() / 100.)
                .collect::<Vec<Vec3>>()
        })
    }
}

fn to_ivec3(val: &Vec3) -> IVec3 {
    IVec3::new(
        (100. * val.x).round() as i32,
        0,
        (100. * val.z).round() as i32,
    )
}
