mod inputs;
fn main() {
    
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_2() -> i32 {
    let input = inputs::input().split("\n\n");
    let mut cards: Vec<BingoCard> = vec![];
    let mut zero = true;
    let mut called_numbers: Vec<i32> = vec![];
    let mut last_winning_score = 0;
    for card in input {
        if zero {
            called_numbers = get_numbers_arr(card);
            zero = false;
        } else {
            cards.push(BingoCard::new(card));
        }
    }

    for num in called_numbers {
        for card in cards.iter_mut() {
            if card.update(num) {
                last_winning_score = card.score();
            }
        }
    }
    last_winning_score
}

fn part_1() -> i32 {
    let input = inputs::input().split("\n\n");
    let mut cards: Vec<BingoCard> = vec![];
    let mut zero = true;
    let mut called_numbers: Vec<i32> = vec![];
    for card in input {
        if zero {
            called_numbers = get_numbers_arr(card);
            zero = false;
        } else {
            cards.push(BingoCard::new(card));
        }
    }

    for num in called_numbers {
        for card in cards.iter_mut() {
            if card.update(num) {
                //card.pretty_print();
                return card.score();
            }
        }
    }
    -1
}

fn get_numbers_arr(input: &str) -> Vec<i32> {
    let nums = input.split(',');
    let mut res: Vec<i32> = Vec::with_capacity(input.split(',').count());
    for num in nums {
        res.push(num.parse().unwrap());
    }
    res

}

struct BingoCard {
    nums: Vec<i32>,
    marked: Vec<bool>,
    last_score: i32,
    has_won: bool,
}

impl BingoCard {
    fn new(input: &str) -> BingoCard {
        let mut res: Vec<i32> = Vec::with_capacity(5*5);
        let nums = input.split('\n');
        for line in nums {
            for num in line.split(' ') {
                if let Ok(i) = num.parse() { 
                    res.push(i);
                };
            }
            
        }
        BingoCard {
            nums: res,
            marked: vec![false; 5*5],
            last_score: 0,
            has_won: false
        }
    }

    #[allow(dead_code)]
    fn pretty_print(&self) {
        for i in 0..25 {
            if i % 5 == 0 {
                println!("\n");
            }
            if self.marked[i] {
                print!("*{}* ", self.nums[i]);
            } else {
                print!("{} ", self.nums[i]);
            }
            
        }
    }
    #[allow(dead_code)]
    fn get(&self, x: usize, y: usize) -> i32 {
        self.nums[5*x + y]
    }

    fn pos_of_element(&self, element: i32) -> Option<usize> {
        self.nums.iter().position(|&x| x== element)
    }

    #[allow(dead_code)]
    fn is_marked(&self, element: i32) -> bool {
        match self.pos_of_element(element) {
            Some(e) => self.marked[e],
            None => false,
        }
    }

    fn update(&mut self, next: i32) -> bool {
        if self.has_won {
            return false;
        }
        match self.pos_of_element(next) {
            None => false,
            Some(pos) => {
                self.last_score = next;
                self.marked[pos] = true;
                if self.win() {
                    self.has_won = true;
                    true
                } else {
                    false
                }
            }
        }
    }

    fn score(&self) -> i32 {
        self.sum_unmarked() * self.last_score
    }

    fn sum_unmarked(&self) -> i32 {
        let mut res = 0;
        for i in 0..25 {
            if !self.marked[i] {
                res += self.nums[i];
            }
        }
        res
    }

    fn win(&self) -> bool {
        let winning_frames: Vec<Vec<usize>> = vec![
            vec![0 , 1, 2, 3, 4],
            vec![5 , 6, 7, 8, 9],
            vec![10, 11, 12, 13, 14],
            vec![15, 16, 17, 18, 19],
            vec![20, 21, 22, 23, 24],
            vec![0, 5, 10, 15, 20],
            vec![1, 6, 11, 16, 21],
            vec![2, 7, 12, 17, 22],
            vec![3, 8, 13, 18, 23],
            vec![4, 9, 14, 19, 24],
            //vec![0, 6, 12, 18, 24],
            //vec![4, 8, 12, 16, 20],
        ];

        for frame in winning_frames {
            let mut guard = true;
            let _dbg = frame.clone();
            for idx in frame {
                if !self.marked[idx] {
                    guard = false;
                    break;
                }
            }
            if guard {
                return true;
            }
        }
        false
    }
}


#[cfg(test)]
mod tests {
    use crate::inputs::test;
    use crate::get_numbers_arr;
    use crate::BingoCard;

    #[test]
    fn default_test() {
        assert_eq!(true, true);
    }

    #[test]
    fn parse_numbers_arr() {
        let test = test();
        let arr = get_numbers_arr(test.split("\n\n").nth(0).unwrap());
        assert_eq!(arr[0], 7);
        assert_eq!(arr[6], 23);
    }

    #[test]
    fn parse_card() {
        let test = test();
        let cardstr = test.split("\n\n").nth(1).unwrap();
        let output = BingoCard::new(cardstr);
        assert_eq!(output.get(0,0), 22);
        assert_eq!(output.get(1,1), 2);
        assert_eq!(output.get(2,2), 14);
        assert_eq!(output.get(3,3), 18);
        assert_eq!(output.get(4,4), 19);
        assert_eq!(output.get(2,1), 9);
    }

    #[test]
    fn check_winner() {
        let test = test();
        let cardstr = test.split("\n\n").nth(3).unwrap();
        let mut output = BingoCard::new(cardstr);
        assert_eq!(output.get(0,0), 14);
        assert_eq!(output.pos_of_element(14), Some(0));

        for e in [7,4,9,5,11,17,23,2,0,14,21] {
            assert_eq!(output.update(e), false);
            assert_eq!(output.is_marked(e), true);
        }
        assert_eq!(output.update(24), true);
    }

    #[test]
    fn score_winner() {
        let test = test();
        let cardstr = test.split("\n\n").nth(3).unwrap();
        let mut output = BingoCard::new(cardstr);

        for e in [7,4,9,5,11,17,23,2,0,14,21] {
            output.update(e);
        }
        assert_eq!(output.update(24), true);

        assert_eq!(output.sum_unmarked(), 188);
        assert_eq!(output.score(), 4512);
    }

    #[test]
    fn part_1_test() {
        let test = test().split("\n\n");
        let mut cards: Vec<BingoCard> = vec![];
        let mut zero = true;
        let mut called_numbers: Vec<i32> = vec![];
        for card in test {
            if zero {
                called_numbers = get_numbers_arr(card);
                zero = false;
            } else {
                cards.push(BingoCard::new(card));
            }
        }

        for num in called_numbers {
            for card in cards.iter_mut() {
                if card.update(num) {
                    assert_eq!(4512, card.score());
                    return;
                }
            }
        }
    }

    #[test]
    fn part_2_test() {
        let test = test().split("\n\n");
        let mut cards: Vec<BingoCard> = vec![];
        let mut zero = true;
        let mut called_numbers: Vec<i32> = vec![];
        let mut last_winning_score = 0;
        for card in test {
            if zero {
                called_numbers = get_numbers_arr(card);
                zero = false;
            } else {
                cards.push(BingoCard::new(card));
            }
        }

        for num in called_numbers {
            for card in cards.iter_mut() {
                if card.update(num) {
                    last_winning_score = card.score();
                }
            }
        }
        assert_eq!(last_winning_score, 1924);
    }
}
