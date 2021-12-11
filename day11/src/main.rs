mod inputs;
fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
    
}

fn part_1() -> i32 {
    let mut input = Map::new(inputs::input());
    let mut acc = 0;
    for _ in 0..100 {
        acc += input.run_simulation();
    }
    acc
}

fn part_2() -> i32 {
    let mut input = Map::new(inputs::input());
    let mut i = 0;
    loop {
        i += 1;
        if input.run_simulation() == 100 {
            return i;
        }
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
struct Coord {
    // note X and Y are incorrect here, 
    // I realised this too far in to refactor everything sorry
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Map {
    map: Vec<Vec<Jelly>>,
    height: usize,
    width: usize,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Jelly {
    power: u32,
    state: FlashState,
}

impl Jelly {
    fn power_up(&mut self) -> bool {
        //returns True if this causes it to flash
        if self.state == FlashState::HasNotFlashed {
            match self.power {
                0..=8 => {
                    self.power += 1;
                    return false
                },
                9 => {
                    self.state = FlashState::WillFlash;
                    return true
                },
                _ => { 
                    panic!("this shouldn't happen");
                }
            }
        }
        false
        
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
enum FlashState {
    WillFlash,
    HasFlashed,
    HasNotFlashed,
}

impl Map {
    fn _print(&self) {
        for x in 0..self.map.len() { // reset state
            for y in 0..self.map[x].len() {
                print!("{}",self.map[x][y].power);
            }
            println!();
        }
    }
    fn new(map_string: &str) -> Map {
        let map_lines = map_string.lines();
        let height = map_string.lines().count();
        let width = map_string.lines().next().unwrap().chars().count();
        let mut map = Vec::with_capacity(height);
        for line in map_lines {
            let mut next = Vec::with_capacity(width);
            for num in line.chars() {
                next.push(Jelly { power: num.to_digit(10).unwrap(), state: FlashState::HasNotFlashed });
            }
            map.push(next);
        }
        Map {
            map,
            height,
            width,
        }
    }

    fn at (&self, coords: Coord) -> Jelly {
        self.map[coords.x][coords.y]
    }

    fn get_neighbours_coords(&self, coords: Coord) -> Vec<Coord> {
        let mut res = Vec::with_capacity(8);
        let x = coords.x;
        let y = coords.y;
        if x != 0 {
            res.push(Coord{ x: x - 1 , y}); // N
        }
        if x != 0 && y != 0 {
            res.push(Coord{ x: x - 1 , y: y - 1}); // NE
        }
        if y != 0 {
            res.push(Coord{ x, y: y - 1}); // E
        }
        if y != 0 && x+1 != self.height { // SE
            res.push(Coord{ x: x+1, y: y -1 })
        }
        if x+1 != self.height {
            res.push(Coord{ x: x + 1, y}); // S
        }
        if x+1 != self.height && y+1 != self.width { // SW
            res.push(Coord{ x: x + 1, y: y + 1});
        }
        if y+1 != self.width {
            res.push(Coord{ x, y: y + 1}); // W
        }
        if x != 0 && y+1 != self.width {
            res.push(Coord{ x: x - 1, y: y + 1}); // NW
        }

        res
    }

    fn run_simulation(&mut self) -> i32 {
        for x in 0..self.map.len() { // power everything up once
            for y in 0..self.map[x].len() {
                self.map[x][y].power_up();
            }
        }
        // state based actions
        let mut keep_looping = true;
        while keep_looping {
            keep_looping = false;
            for x in 0..self.map.len() {
                for y in 0..self.map[x].len() {
                    if self.map[x][y].state == FlashState::WillFlash {
                        for neigh in self.get_neighbours_coords(Coord{x,y}) {
                            let mut new_jel = self.at(neigh); // for some reason incrementing through map didn't work
                            if new_jel.power_up() {
                                keep_looping = true; // restarts the loop afterwards as at least 1 state has changed
                            }
                            self.map[neigh.x][neigh.y] = new_jel;
                        }
                        self.map[x][y].state = FlashState::HasFlashed;
                    }  
                }
            }
        }

        let mut res = 0;
        for x in 0..self.map.len() { // reset state and count flashes
            for y in 0..self.map[x].len() {
                match self.map[x][y].state {
                    FlashState::HasFlashed => {
                        res += 1;
                        self.map[x][y].state = FlashState::HasNotFlashed;
                        self.map[x][y].power = 0;
                    },
                    FlashState::HasNotFlashed => (),
                    FlashState::WillFlash => {
                        panic!("fatal: a jelly with the state WillFlash exists in the reset state part of the loop");
                    }
                }
               
            }
        }

        res 
    }

}

#[cfg(test)]
mod test {
    use crate::inputs;
    use crate::*;

    #[test]
    fn first_simulation() {
        let mut test = Map::new(inputs::test());

        test.run_simulation();
        assert_eq!(test.map, Map::new(inputs::test_after_1()).map);
        
    }

    #[test]
    fn inc() {
        let mut j_test = Jelly{power: 9, state: FlashState::HasNotFlashed};
        j_test.power_up();
        assert_eq!(j_test.state, FlashState::WillFlash);
        let mut j_test_1 = Jelly{power: 2, state: FlashState::HasNotFlashed};
        j_test_1.power_up();
        assert_eq!(j_test_1.power, 3);

    }

    #[test]
    fn second_simulation() {
        let mut test = Map::new(inputs::test());

        test.run_simulation();
        test.run_simulation();
        assert_eq!(test.map, Map::new(inputs::test_after_2()).map);
    }

    #[test]
    fn tenth_siumlation() {
        let mut test = Map::new(inputs::test());
        let mut acc = 0;
        for _ in 0..10 {
            acc += test.run_simulation();
        }

        assert_eq!(test.map, Map::new(inputs::test_after_10()).map);
        assert_eq!(acc, 204);
    }

    #[test]
    fn hundreth_simulation() {
        let mut test = Map::new(inputs::test());
        let mut acc = 0;
        for _ in 0..100 {
            acc += test.run_simulation();
        }
        assert_eq!(acc,1656);
    }


}