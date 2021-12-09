mod inputs;

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> u32 {
    let  hm = HeightMap::new(inputs::input());
    let mut res = 0;
    for x in 0..hm.height {
        for y in 0..hm.width {
            if hm.is_low_point(x,y) {
                res += 1 + hm.map[x][y];
            }
        }
    }
    res
}

fn part_2() -> usize {
    let mut hm = HeightMap::new(inputs::input());
    let mut res = 1;
    hm.basins.sort_by_key(|k| k.len());
    for _ in 0..3 {
        res *= hm.basins.pop().unwrap().len();
    }
    res
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coord {
    // note X and Y are incorrect here, 
    // I realised this too far in to refactor everything sorry
    x: usize,
    y: usize,
}

struct HeightMap {
    map: Vec<Vec<u32>>,
    height: usize,
    width: usize,
    basins: Vec<Vec<Coord>>,
}

impl HeightMap {
    fn new(map_string: &str) -> HeightMap {
        let basins: Vec<Vec<Coord>> = vec![];
        let map_lines = map_string.lines();
        let height = map_string.lines().count();
        let width = map_string.lines().next().unwrap().chars().count();
        let mut map = Vec::with_capacity(height);
        for line in map_lines {
            let mut next = Vec::with_capacity(width);
            for num in line.chars() {
                next.push(num.to_digit(10).unwrap());
            }
            map.push(next);
        }
        let mut res = HeightMap {
            map,
            height,
            width,
            basins
        };
        res.fill_basins();
        res
    }

    fn _pretty_print(&self)  {
        for x in 0..self.height {
            for y in 0..self.width {
                if self.map[x][y] == 9 {
                    print!("w");
                } else {
                    match self._get_basin_num(Coord { x, y }) {
                        Some(n) => print!("{}", n),
                        None => print!("x"),

                    }
                }
            }
            println!();
        }
            
    }

    fn _get_basin_num(&self, location: Coord) -> Option<usize> {
        for i in 0..self.basins.len() {
            if self.basins[i].contains(&location) {
                return Some(i)
            }
        }
        None
    }

    fn fill_basins(&mut self) { 
        assert!(self.basins.is_empty());
        for x in 0..self.height {
            for y in 0..self.width {
                if self.is_low_point(x,y) {
                    let mut this_basin = vec![Coord{x,y}];
                    let mut to_add = self.get_coords_of_unwalled_neighbours(Coord{x,y});
                    while !to_add.is_empty() {
                        let candidate = to_add.pop().unwrap();
                        if !this_basin.contains(&candidate) {
                            for e in self.get_coords_of_unwalled_neighbours(candidate) {
                                to_add.push(e);
                            }
                            this_basin.push(candidate);
                        }
                    }
                    self.basins.push(this_basin);
                }
            }
        }
    }
    

    fn get_neighbours_coords(&self, coords: Coord) -> Vec<Coord> {
        let mut res = Vec::with_capacity(4);
        let x = coords.x;
        let y = coords.y;
        if coords.x != 0 {
            res.push(Coord{ x: x - 1 , y}); // N
        }
        if y != 0 {
            res.push(Coord{ x, y: y - 1}); // E
        }
        if x+1 != self.height {
            res.push(Coord{ x: x + 1, y}); // S
        }
        if y+1 != self.width {
            res.push(Coord{ x, y: y + 1}); // W
        }

        res
    }

    fn get_neighbours(&self, x: usize, y: usize) -> Vec<u32> {
        let mut res = Vec::with_capacity(4);
        if x != 0 { 
            res.push(self.map[x - 1][y]); // N
        }
        if y != 0 {
            res.push(self.map[x][y - 1]); // E
        }
        if x+1 != self.height {
            res.push(self.map[x + 1][y]); // S
        }
        if y+1 != self.width {
            res.push(self.map[x][y + 1]); // W 
        }
        res
    }

    fn is_low_point(&self, x: usize, y: usize) -> bool {
        for neighbour in self.get_neighbours(x, y) {
            if self.map[x][y] >= neighbour  {
                return false
            }
        }
        true
    }

    fn get_by_coords(&self, location: Coord) -> u32 {
        self.map[location.x][location.y]

    }

    fn get_coords_of_unwalled_neighbours(&self, location: Coord) -> Vec<Coord> {
        let mut res = Vec::with_capacity(4);
        for neighbour in self.get_neighbours_coords(location) {
            if self.get_by_coords(neighbour) != 9  {
                res.push(neighbour);
            }
        }
        res

    }
}

#[cfg(test)]
mod test {
    use crate::inputs;
    use crate::HeightMap;
    use crate::Coord;

    #[test]
    fn create_struct() {
        let test = HeightMap::new(inputs::test());
        assert_eq!(test.map[0][0], 2);
        assert_eq!(test.map[0][1], 1);
        assert_eq!(test.map[1][0], 3);
        assert_eq!(test.map[1][1], 9);
        assert_eq!(test.map[4][9], 8);
        assert_eq!(test.height, 5);
        assert_eq!(test.width, 10)
    }

    #[test]
    fn find_neighbours() {
        let test = HeightMap::new(inputs::test());
        assert_eq!(test.get_neighbours(1,1), vec![1, 3, 8, 8]); // array of neighbours going clockwise
        assert_eq!(test.get_neighbours(1,2), vec![9, 9, 5, 7]); 
    }

    #[test]
    fn find_neighbours_on_edges() {
        let test = HeightMap::new(inputs::test());
        assert_eq!(test.get_neighbours(0,0), vec![3, 1]);
        assert_eq!(test.get_neighbours(0,1), vec![2, 9, 9]);
        assert_eq!(test.get_neighbours(4,0), vec![8, 8]); 
        assert_eq!(test.get_neighbours(4,9), vec![9, 7]); 
    }

    #[test]
    fn find_neighbours_coordinates() {
        let test = HeightMap::new(inputs::test());

        assert_eq!(test.get_neighbours_coords(Coord{x: 0, y: 0}), vec![Coord{x: 1, y: 0}, Coord{x: 0, y: 1}]);


    }

    #[test]
    fn find_low_point() {
        let test = HeightMap::new(inputs::test());
        assert_eq!(test.is_low_point(0, 0), false);
        assert_eq!(test.is_low_point(0, 1), true);
    }

}