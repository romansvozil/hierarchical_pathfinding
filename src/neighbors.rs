//! A crate with the most common Neighborhoods

use crate::Point;
use std::fmt::Debug;

/// Defines how a Path can move along the Grid.
///
/// Different use cases may have different conditions for Paths.
/// For example if movement is restricted to the 4 cardinal directions, any Paths generated should
/// reflect that by only containing those steps.
///
/// This Trait is a generalized solution to that problem. It provides a function to query all
/// neighboring Points of an existing Point and a Heuristic for how long it might take to reach
/// a goal from a Point.
///
/// The most common implementations of this Trait are already provided by this Module:
/// - [`ManhattanNeighborhood`] for Agents that can move
/// up, down, left or right
/// - [`MooreNeighborhood`] for Agents that can move
/// up, down, left, right, as well as the 4 diagonals (up-right, ...)
pub trait Neighborhood: Clone + Debug {
    /// Provides all the Neighbors of a Point.
    ///
    /// The Neighbors should be written into `target`
    ///
    /// Note that it is not necessary to check weather the Tile at a Point is solid or not.
    /// That check is done later.
    fn get_all_neighbors(&self, point: Point, target: &mut Vec<Point>);
    /// Gives a Heuristic for how long it takes to reach `goal` from `point`.
    ///
    /// This is usually the Distance between the two Points in the Metric of your Neighborhood.
    ///
    /// The returned value _**must be**_ less than or equal to the real cost of walking from
    /// `point` to `goal`. See [A*](https://en.wikipedia.org/wiki/A*_search_algorithm) for details.
    ///
    /// If there is no proper way of calculation how long it takes, simply return 0. This will
    /// increase the time it takes to calculate the Path, but at least it will always be correct.
    fn heuristic(&self, point: Point, goal: Point) -> usize;
}

/// A Neighborhood for Agents moving along the 4 cardinal directions.
///
/// Also known as [Von Neumann Neighborhood](https://en.wikipedia.org/wiki/Von_Neumann_neighborhood),
/// Manhattan Metric or [Taxicab Geometry](https://en.wikipedia.org/wiki/Taxicab_geometry).
///
/// ```no_code
/// A: Agent, o: reachable in one step
///   o
///   |
/// o-A-o
///   |
///   o
/// ```
#[derive(Clone, Copy, Debug)]
pub struct ManhattanNeighborhood {
    width: usize,
    height: usize,
}

impl ManhattanNeighborhood {
    /// Creates a new ManhattanNeighborhood.
    ///
    /// `width` and `height` are the size of the Grid to move on.
    pub fn new(width: usize, height: usize) -> ManhattanNeighborhood {
        ManhattanNeighborhood { width, height }
    }
}

impl Neighborhood for ManhattanNeighborhood {
    fn get_all_neighbors(&self, point: Point, target: &mut Vec<Point>) {
        let (width, height) = (self.width, self.height);

        #[rustfmt::skip]
        static ALL_DELTAS: [(isize, isize); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

        for (dx, dy) in ALL_DELTAS.iter() {
            let x = point.0 as isize + dx;
            let y = point.1 as isize + dy;
            if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                target.push((x as usize, y as usize))
            }
        }
    }
    fn heuristic(&self, point: Point, goal: Point) -> usize {
        let diff_0 = if goal.0 > point.0 {
            goal.0 - point.0
        } else {
            point.0 - goal.0
        };
        let diff_1 = if goal.1 > point.1 {
            goal.1 - point.1
        } else {
            point.1 - goal.1
        };
        diff_0 + diff_1
    }
}

/// A Neighborhood for Agents moving along the 4 cardinal directions and the 4 diagonals.
///
/// Also known as [Moore Neighborhood](https://en.wikipedia.org/wiki/Moore_neighborhood),
/// [Maximum Metric](https://en.wikipedia.org/wiki/Chebyshev_distance) or Chebyshev Metric.
///
/// ```no_code
/// A: Agent, o: reachable in one step
/// o o o
///  \|/
/// o-A-o
///  /|\
/// o o o
/// ```
#[derive(Clone, Copy, Debug)]
pub struct MooreNeighborhood {
    width: usize,
    height: usize,
}

impl MooreNeighborhood {
    /// Creates a new MooreNeighborhood.
    ///
    /// `width` and `height` are the size of the Grid to move on.
    pub fn new(width: usize, height: usize) -> MooreNeighborhood {
        MooreNeighborhood { width, height }
    }
}

impl Neighborhood for MooreNeighborhood {
    fn get_all_neighbors(&self, point: Point, target: &mut Vec<Point>) {
        let (width, height) = (self.width, self.height);

        #[rustfmt::skip]
        static ALL_DELTAS: [(isize, isize); 8] = [(0, -1), (1, -1), (1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1)];

        for (dx, dy) in ALL_DELTAS.iter() {
            let x = point.0 as isize + dx;
            let y = point.1 as isize + dy;
            if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                target.push((x as usize, y as usize))
            }
        }
    }
    fn heuristic(&self, point: Point, goal: Point) -> usize {
        let diff_0 = if goal.0 > point.0 {
            goal.0 - point.0
        } else {
            point.0 - goal.0
        };
        let diff_1 = if goal.1 > point.1 {
            goal.1 - point.1
        } else {
            point.1 - goal.1
        };
        
        (((diff_0 * diff_0 + diff_1 * diff_1) as f32).sqrt() * 2.) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod manhattan {
        use super::*;
        #[test]
        fn get_all_neighbors() {
            let neighborhood = ManhattanNeighborhood::new(5, 5);
            let mut target = vec![];
            neighborhood.get_all_neighbors((0, 2), &mut target);
            assert_eq!(target, vec![(0, 1), (1, 2), (0, 3)],);
        }

        #[test]
        fn heuristic() {
            let neighborhood = ManhattanNeighborhood::new(5, 5);
            assert_eq!(neighborhood.heuristic((3, 1), (0, 0)), 3 + 1);
        }
    }
    mod moore {
        use super::*;

        #[test]
        fn get_all_neighbors() {
            let neighborhood = MooreNeighborhood::new(5, 5);
            let mut target = vec![];
            neighborhood.get_all_neighbors((0, 2), &mut target);
            assert_eq!(target, vec![(0, 1), (1, 1), (1, 2), (1, 3), (0, 3)],);
        }

        #[test]
        fn heuristic() {
            // let neighborhood = MooreNeighborhood::new(5, 5);
            // assert_eq!(neighborhood.heuristic((3, 1), (0, 0)), 3);
        }
    }
}
