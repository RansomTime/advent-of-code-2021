use itertools::Itertools;

fn main() {
    println!("part 1: {}", pt_1());
    println!("part 2: {}", pt_2());

}



fn pt_1() -> i32 {
    let codes = inputs::input();
    let mut res = 0;
    for entry in codes.split('\n'){
        res += DisplayEntry::new(entry).get_known_digits();
    }
    res
}

fn pt_2() -> usize {
    let codes = inputs::input();
    let mut res = 0;
    for entry in codes.split('\n'){
        res += DisplayEntry::new(entry).get_integer_output();
    }

    res
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct DisplayEntry {
    patterns: &'static str,
    output: &'static str,
}

impl DisplayEntry {
    fn new(input: &'static str) -> DisplayEntry {
        let mut res = input.split(" | ");
        DisplayEntry {
            patterns: res.next().unwrap(),
            output: res.next().unwrap(),
        }
    }

    fn get_known_digits(&self) -> i32 {
        let mut res = 0;
        for e in self.output.split(' ') {
            match e.len() {
                2 | 3 | 4 | 7 => res += 1,
                _  => (),
            }
        }
        res
    }

    fn get_string_of_known_digit(&self, digits_in_output: usize) -> &str {
        for e in self.patterns.split(' ') {
            if e.chars().count() == digits_in_output {
                return e
            }
        }
        panic!("invalid string parsed: no unit of {} found in string {}", digits_in_output, self.patterns)
    }

    fn get_array_of_digits(&self, digits_in_output: usize) -> Vec<&str> {
        let mut res = vec![];
        for e in self.patterns.split(' ') {
            if e.chars().count() == digits_in_output {
                res.push(e);
            }
        }
        res
    }
}

impl DisplayEntry {

    fn _find_only_string_that_contains(arr: &[&str], needle: &str) -> usize {
        // returns the location in arr of the only string that contains all of needle
        let mut found = vec![];
        for i in 0..arr.len() {
            let mut guard = true;
            for char in needle.chars() {
                if !arr[i].contains(char) {
                    guard = false;
                }
            }
            if guard {
                found.push(i);
            }
        }
        assert_eq!(found.len(), 1);
        found.pop().unwrap()
    }

    fn _find_only_string_that_does_not_contain(arr: &[&str], needle: &str) -> usize {
        // returns the location in arr of the only string that does not contain all of needle
        let mut found = 255;
        for i in 0..arr.len() {
            for char in needle.chars() {
                if arr[i].contains(char) {
                    continue;
                } else {
                    found = i;
                }
            }

        }
        found
    }


    fn get_display(&self) -> [String;10]{
        let mut digits :[String; 10] = Default::default();
    
        digits[1] = String::from(self.get_string_of_known_digit(2));
        digits[4] = String::from(self.get_string_of_known_digit(4));
        digits[7] = String::from(self.get_string_of_known_digit(3));
        digits[8] = String::from(self.get_string_of_known_digit(7));
    
        let mut five_digits = self.get_array_of_digits(5); // 2, 3 or 5
        let mut six_digits = self.get_array_of_digits(6);  // 0, 6 or 9
        {
            let three_pos = DisplayEntry::_find_only_string_that_contains(&five_digits, &digits[1]);
            digits[3] = String::from(five_digits[three_pos]);
            five_digits.remove(three_pos);

            let six_pos = DisplayEntry::_find_only_string_that_does_not_contain(&six_digits, &digits[1]);
            digits[6] = String::from(six_digits[six_pos]);
            six_digits.remove(six_pos);

            // 2, 5
            // 0, 9
            // out of 0 and 9 - 9 does have 4
            let nine_pos = DisplayEntry::_find_only_string_that_contains(&six_digits, &digits[4]);
            digits[9] = String::from(six_digits[nine_pos]);
            six_digits.remove(nine_pos);
            digits[0] = String::from(six_digits.pop().unwrap());

            // 2, 5 
            // find c
            let mut c = 'x';
            for digit in ['a','b','c','d','e','f','g'] {
                if !(digits[9].chars().contains(&digit)) {
                    c = digit;
                }
            }
            let two_pos = DisplayEntry::_find_only_string_that_contains(&five_digits, &String::from(c));
            digits[2] = String::from(five_digits[two_pos]);
            five_digits.remove(two_pos);
            digits[5] = String::from(five_digits.pop().unwrap());
        }
        digits
    }

    fn get_integer_output(&self) -> usize {
        let mut digits = self.get_display();
        for i in 0..digits.len() {
            digits[i] = digits[i].chars().sorted().collect::<String>(); // grr
        }
        let mut mult = 1000;
        let mut res = 0;
        for e in self.output.split(' ') {
            let f = e.chars().sorted().collect::<String>(); // grrrrrrr
            for i in 0..digits.len() {
                if digits[i] == f {
                    res += i*mult;
                    mult /= 10;
                }
            }
        }
        res
    }
}
mod inputs;
#[cfg(test)]
mod test {
    use crate::DisplayEntry;
    use crate::inputs;


    #[test]
    fn parse_basic_string() {
        let test = DisplayEntry::new("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        assert_eq!(test.patterns, "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb");
        assert_eq!(test.output, "fdgacbe cefdb cefbgd gcbe");
    }

    #[test]
    fn get_easy_digits() {
        let test = DisplayEntry::new("be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe");
        assert_eq!(test.get_known_digits(), 2)
    }


    #[test]
    fn pt_1_test() {
        let test = inputs::test();
        let mut res = 0;
        for entry in test.split('\n'){
            res += DisplayEntry::new(entry).get_known_digits();
        }

        assert_eq!(res, 26);
    }

    #[test]
    fn part_2_finds_digits() {

        let digits = DisplayEntry::new("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf").get_display();
    
        assert_eq!(digits[0],"cagedb");
        assert_eq!(digits[1],"ab");
        assert_eq!(digits[2],"gcdfa");
        assert_eq!(digits[3],"fbcad");
        assert_eq!(digits[4],"eafb");
        assert_eq!(digits[5],"cdfbe");
        assert_eq!(digits[6],"cdfgeb");
        assert_eq!(digits[7],"dab");
        assert_eq!(digits[8],"acedgfb");
        assert_eq!(digits[9],"cefabd");
    }

    #[test]
    fn part_2_numbers_out() {
        let de = DisplayEntry::new("acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab | cdfeb fcadb cdfeb cdbaf");
        
        assert_eq!(de.get_integer_output(),5353);
        assert_eq!(DisplayEntry::new("fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb").get_integer_output(), 9361)
    }

    #[test]
    fn find_strings() {
        assert_eq!(DisplayEntry::_find_only_string_that_does_not_contain(&vec!["1", "12", "23"], &String::from("2")), 0);
        assert_eq!(DisplayEntry::_find_only_string_that_does_not_contain(&vec!["12", "123", "234","134"], &String::from("1")), 2);
    }

    #[test]
    fn part_2_test() {
        let codes = inputs::test();
        let mut res = 0;
        for entry in codes.split('\n'){
            res += DisplayEntry::new(entry).get_integer_output();
        }

        assert_eq!(res, 61229);
    }
}