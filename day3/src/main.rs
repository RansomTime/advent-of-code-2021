use std::convert::TryInto;
mod inputs;
fn main() {
    println!("part 1: {}",part_1(inputs::input()).consumption());
    let pt2 = part_2(inputs::input());
    println!("part 2: {}", pt2.oxygen * pt2.co2 );
}

struct BitCounter {
    zero: u32,
    one: u32,
}

impl BitCounter {
    fn new() -> BitCounter {
        BitCounter{
            zero: 0,
            one: 0,
        }
    }

    fn get_count(&self, digit: char)  -> usize {
        match digit {
            '0' => self.zero.try_into().unwrap(),
            '1' => self.one.try_into().unwrap(),
            _ => panic!("unexpected digit {}", digit)
        }
    }

    fn most(&self) -> char {
        if self.one >= self.zero  {
            '1'
        } else {
            '0'
        }
    }


    fn least(&self) -> char {
        if self.zero <= self.one {
            '0'
        } else {
            '1'
        }
    }

    fn count(&mut self, digit: char) {
        match digit {
            '0' => self.zero += 1,
            '1' => self.one += 1,
            _ => panic!("unexpected digt {}", digit)
        }
    }

    
}

struct Rates {
    gamma: u32,
    max_int: u32
}

impl Rates {
    fn epsilon(&self) -> u32 {
        self.max_int - self.gamma
    }

    fn consumption(&self) -> u32 {
        self.epsilon() * self.gamma
    }

}

fn part_1 (measurements: Vec<&str>) -> Rates {
    let word_size: usize  = measurements[0].chars().count();
    let mut counters =Vec::with_capacity(word_size);
    for _ in 0..word_size{
        counters.push(BitCounter::new());
    };

    for measurement in measurements.iter() {
        for i in 0..word_size {
            let next = measurement.chars().nth(i).unwrap();
            match next {
                '0' => counters[i].zero += 1,
                '1' => counters[i].one += 1,
                _ => println!("Unexpected character: {}", next),
            }
        }
    }
    let two: u32 = 2;
    let mut res = Rates {
        gamma: 0,
        max_int: two.pow(word_size.try_into().unwrap()) - 1
    };
    
    for i in 0..word_size {
        if counters[i].most() == '1' {
            res.gamma += two.pow((word_size-(i+1)).try_into().unwrap());
        }
    }


    res
}

fn binary_string_to_u32(binary: &str) -> u32 {
    let word_size = binary.chars().count();
    let two: u32 = 2;
    let mut res = 0;
    for i in 0..word_size {
        if binary.chars().nth(i).unwrap() == '1' {
            res += two.pow((word_size-(i+1)).try_into().unwrap());
        }        
    }
    res
}

struct Ratings {
    oxygen: u32,
    co2: u32
}

fn part_2 (measurements: Vec<&str>) -> Ratings {

    Ratings {
        oxygen: part_2_recursive(measurements.clone(), 0, true),
        co2: part_2_recursive(measurements, 0, false),
    }
}


fn part_2_recursive (measurements: Vec<&str>, idx: usize, filter_oxygen: bool) -> u32 {
    let mut counter = BitCounter::new();
    for measurement in measurements.iter() {
        counter.count(measurement.chars().nth(idx).unwrap());
    }

    let filter;
    if filter_oxygen {
        filter = counter.most();
    } else {
        filter = counter.least();
    }
    

    let mut next: Vec<&str> = Vec::with_capacity(counter.get_count(filter));

    for measurement in measurements.iter() {
        if measurement.chars().nth(idx).unwrap() == filter {
            next.push(measurement);
        }
    }

    if next.len() == 1 {
        binary_string_to_u32(next[0])
    } else {
        part_2_recursive(next, idx + 1, filter_oxygen)
    }
}



#[cfg(test)]
mod tests {
    use crate::*;
 
    
    #[test]
    fn part_1_tests() {
    let test = inputs::test();

    let test_result = part_1(test);
    assert_eq!(test_result.epsilon(), 9);
    assert_eq!(test_result.gamma, 22);
    assert_eq!(test_result.consumption(), 198);
    }

    #[test]
    fn oxy_raiting_determines_most_common() {
        let test = BitCounter { 
            zero: 5,
            one: 7
        };
        assert_eq!(test.most(), '1');
    }

    #[test]
    fn oxy_raiting_determines_most_common_with_zero() {
        let test = BitCounter { 
            zero: 4,
            one: 3
        };
        assert_eq!(test.most(), '0');
    }

    #[test]
    fn oxy_raiting_returns_1_on_tie() {
        let test = BitCounter { 
            zero: 1,
            one: 1
        };

        assert_eq!(test.most(), '1');
    }


    #[test]
    fn co2_rating_determines_least_common() {
        let test = BitCounter { 
            zero: 5,
            one: 7
        };
        assert_eq!(test.least(), '0');
    }

    #[test]
    fn co2_rating_determines_least_common_with_one() {
        let test = BitCounter { 
            zero: 3,
            one: 2
        };
        assert_eq!(test.least(), '1');
    }

    #[test]
    fn co2_raiting_returns_0_on_tie() {
        let test = BitCounter { 
            zero: 1,
            one: 1
        };

        assert_eq!(test.least(), '0');
    }

    #[test]
    fn binary_decode() {
        assert_eq!(binary_string_to_u32("0"),0);
        assert_eq!(binary_string_to_u32("1"),1);
        assert_eq!(binary_string_to_u32("10"),2);
        assert_eq!(binary_string_to_u32("10111"),23);
        assert_eq!(binary_string_to_u32("01010"),10);
    }

    #[test]
    fn bit_coutner_can_count_zeroes() {
        let mut test = BitCounter::new();
        test.count('0');
        assert_eq!(test.zero, 1);
    }

    #[test]
    fn bit_coutner_can_count_ones() {
        let mut test = BitCounter::new();
        test.count('1');
        assert_eq!(test.one, 1);
    }

    #[test]
    fn bit_coutner_can_count_a_lot() {
        let mut test = BitCounter::new();
        test.count('0');
        test.count('0');
        test.count('1');
        test.count('0');
        test.count('1');
        test.count('0');
        assert_eq!(test.zero, 4);
        assert_eq!(test.one, 2);
    }

    

    #[test]
    fn part_2_tests() {
        let test = inputs::test();

        let part_2_result = part_2(test);

        assert_eq!(part_2_result.oxygen, 23);
        assert_eq!(part_2_result.co2, 10);
        assert_eq!(part_2_result.oxygen * part_2_result.co2, 230);

    }
}