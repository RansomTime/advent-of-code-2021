mod inputs;
use std::ops::Add;
use std::ops::Sub;
use std::convert::TryInto;


fn main() {
    println!("Part 2: {}", part_2(inputs::input()));
}

fn part_2(input: &str) -> u32 {

    let mut lines: Vec<Line> = vec![];
    for e in input.split('\n') {
        lines.push(Line::new(e));
    }

    let mut grid = Grid::new(1000);

    for line in lines {
        grid.add_line(line);
    }

    let mut res = 0;
    for x in 0..grid.size {
        for y in 0..grid.size{
            if grid.grid[x][y] > 1 {
                res += 1;
            }
        }
    }

    res
    
}

#[derive(Debug)]
struct Line {
    p1: Point,
    p2: Point,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}


impl Point {
    fn new(input: &str) -> Point {
        let mut xy = input.split(',');
        Point{ 
            x: xy.next().unwrap().parse().unwrap(),
            y: xy.next().unwrap().parse().unwrap(),
        }
    }

    
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y -other.y,
        }
    }
}

impl Line {
    fn new(input: &str) -> Line {
        let mut points = input.split(" -> ");
        Line { 
            p1: Point::new(points.next().unwrap()),
            p2: Point::new(points.next().unwrap()),
        }     
    }

    fn _is_straight(&self) -> bool {
        self.p1.x == self.p2.x || self.p1.y == self.p2.y
    }

    fn get_pts(&self) -> Vec<Point> {
        let mut res = vec![];
        let mut diff = self.p2 - self.p1;

        if diff.x >= 1 {
            diff.x = 1;
        }
        if diff.x <= -1 {
            diff.x = -1;
        }
        if diff.y >= 1 {
            diff.y = 1;
        }
        if diff.y <= -1 {
            diff.y = -1;
        }

        let mut next: Point = self.p1;
        res.push(next);
        while next != self.p2 {
            next = next + diff;
            res.push(next)
        }
        res
    }
}

#[derive(Debug)]
struct Grid {
    grid: Vec<Vec<u8>>,
    size: usize,
}

impl Grid {
    fn new(size: usize) -> Grid {
        let mut res: Vec<Vec<u8>> = Vec::with_capacity(size);
        for _ in 0..size {
            let mut inner: Vec<u8> = Vec::with_capacity(size);
            for _ in 0..size {
                inner.push(0);
            }
            res.push(inner);
        }

        Grid{
            grid: res,
            size
        }

    }

    fn add_line(&mut self, line: Line) {
        for point in line.get_pts() {
            let x: usize = point.x.try_into().unwrap();
            let y: usize = point.y.try_into().unwrap();
            self.grid[x][y] += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use crate::Line;
    use crate::Point;
    use crate::Grid;
    use crate::inputs;
    #[test]
    fn default_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn make_point() {
        let test = "0,9";
        let point = Point::new(test);

        assert_eq!(point.x, 0);
        assert_eq!(point.y, 9);
    }

    #[test]
    fn make_line() {

        let line = Line::new("0,9 -> 5,9");

        assert_eq!(line.p1.x, 0);
        assert_eq!(line.p1.y, 9);
        assert_eq!(line.p2.x, 5);
        assert_eq!(line.p2.y, 9);
    }

    #[test]
    fn straight_lines() {
        assert_eq!(Line::new("2,2 -> 2,1")._is_straight(), true);
        assert_eq!(Line::new("0,9 -> 5,9")._is_straight(), true);
        assert_eq!(Line::new("0,9 -> 2,9")._is_straight(), true);
        assert_eq!(Line::new("9,4 -> 3,4")._is_straight(), true);
        assert_eq!(Line::new("7,0 -> 7,4")._is_straight(), true);
        assert_eq!(Line::new("3,4 -> 1,4")._is_straight(), true);
    }

    #[test]
    fn not_straight_lines() {
        assert_eq!(Line::new("8,0 -> 0,8")._is_straight(), false);
        assert_eq!(Line::new("6,4 -> 2,0")._is_straight(), false);
        assert_eq!(Line::new("0,0 -> 8,8")._is_straight(), false);
        assert_eq!(Line::new("5,5 -> 8,2")._is_straight(), false);
    }

    #[test]
    fn grid_initalise() {
        assert_eq!(Grid::new(10).grid[0][0], 0);
        assert_eq!(Grid::new(10).grid[9][9], 0);
    }

    #[test]
    fn move_point_by_vector() {
        assert_eq!(Point{x: 2, y: 1} + Point{x: 0, y: 1}, Point{x: 2, y: 2});
    }

    #[test]
    fn points_on_line() {
        let line = Line::new("2,2 -> 2,1");
        assert_eq!(line.get_pts(), vec![Point{x: 2, y: 2}, Point{x: 2, y: 1}]);

        let line_2 = Line::new("0,9 -> 5,9");
        assert_eq!(line_2.get_pts(), vec![
            Point{x: 0, y: 9},
            Point{x: 1, y: 9},
            Point{x: 2, y: 9},
            Point{x: 3, y: 9},
            Point{x: 4, y: 9},
            Point{x: 5, y: 9},
            ]);

        let line_3 = Line::new("9,4 -> 3,4");
        assert_eq!(line_3.get_pts(), vec![
            Point{x: 9, y: 4},
            Point{x: 8, y: 4},
            Point{x: 7, y: 4},
            Point{x: 6, y: 4},
            Point{x: 5, y: 4},
            Point{x: 4, y: 4},
            Point{x: 3, y: 4},
            ]);
    }
    #[test]
    fn add_line_to_grid() {
        let mut test_grid = Grid::new(10);
        let line = Line::new("2,2 -> 2,1");

        test_grid.add_line(line);
        assert_eq!(test_grid.grid[2][2], 1);
        assert_eq!(test_grid.grid[2][1], 1);
    }

    #[test]
    fn points_on_line_diag() {
        let line = Line::new("1,1 -> 3,3");
        assert_eq!(line.get_pts(),vec![
            Point{x: 1, y: 1}, 
            Point{x: 2, y: 2},
            Point{x: 3, y: 3},
        ]);

        let line_2 = Line::new("9,7 -> 7,9");
        assert_eq!(line_2.get_pts(),vec![
            Point{x: 9, y: 7}, 
            Point{x: 8, y: 8},
            Point{x: 7, y: 9},
        ]);
    }

    #[test]
    fn add_overlapping_lines_to_grid() {
        let mut test_grid = Grid::new(10);


        test_grid.add_line(Line::new("0,9 -> 5,9"));
        test_grid.add_line(Line::new("0,9 -> 2,9"));
        assert_eq!(test_grid.grid[0][9], 2);
        assert_eq!(test_grid.grid[1][9], 2);
        assert_eq!(test_grid.grid[2][9], 2);
        assert_eq!(test_grid.grid[3][9], 1);
        assert_eq!(test_grid.grid[5][9], 1);
        assert_eq!(test_grid.grid[9][9], 0);
    }
    #[test]
    fn part_2_test() {
        assert_eq!(crate::part_2(inputs::test()), 12);
    }

}