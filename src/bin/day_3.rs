use std::fs;
use std::str::FromStr;

struct Map {
    trees: Vec<Vec<bool>>,
    width: usize,
    height: usize,
}

impl Map {
    fn has_tree(&self, x: usize, y: usize) -> bool {
        self.trees[y][x % self.width]
    }

    fn count_trees(&self, dx: usize, dy: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut tree_count = 0;

        loop {
            if y >= self.height {
                break;
            }
            if self.has_tree(x, y) {
                tree_count += 1
            }
            x += dx;
            y += dy;
        }

        tree_count
    }
}

#[derive(Debug)]
struct MapErr;

impl FromStr for Map {
    type Err = MapErr;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trees: Vec<Vec<bool>> = s
            .split("\n")
            .filter(|s| s.len() > 0)
            .map(|s| s.chars().map(|c| c == '#').collect())
            .collect();
        let width = trees[0].len();
        let height = trees.len();

        Ok(Map {
            trees,
            width,
            height,
        })
    }
}

fn part1(map: &Map) {
    println!("{}", map.count_trees(3, 1));
}

fn part2(map: &Map) {
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let product = slopes
        .into_iter()
        .fold(1, |acc, (dx, dy)| acc * map.count_trees(dx, dy));
    println!("{}", product);
}

fn main() {
    let contents = fs::read_to_string("data/day_3.txt").expect("File must be present");
    let map: Map = contents.parse().unwrap();

    part1(&map);
    part2(&map);
}
