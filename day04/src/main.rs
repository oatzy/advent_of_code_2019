fn check_part1(code: usize) -> bool {
    let mut value = code;
    let mut current;
    let mut previous = 10;
    let mut has_double = false;

    while value > 0 {
        current = value % 10;
        value = value / 10;

        // we're processing digits last to first
        // so we have to reverse the criteria
        // i.e. digits never increase
        if current > previous {
            return false;
        }

        if current == previous {
            has_double = true;
        }
        previous = current;
    }

    has_double
}

fn check_part2(code: usize) -> bool {
    let mut value = code;
    let mut current;
    let mut previous = 10;
    let mut has_double = false;
    let mut run_len = 1;

    while value > 0 {
        current = value % 10;
        value = value / 10;

        // we're processing digits last to first
        // so we have to reverse the criteria
        // i.e. digits never increase
        if current > previous {
            return false;
        }

        if current == previous {
            run_len += 1;
        } else {
            if run_len == 2 {
                has_double = true;
            }
            run_len = 1;
        }
        previous = current;
    }
    // don't forget the boundry
    if run_len == 2 {
        has_double = true;
    }

    has_double
}

fn main() {
    let part1 = (193651..649729).filter(|&x| check_part1(x)).count();
    println!("{}", part1);

    let part2 = (193651..649729).filter(|&x| check_part2(x)).count();
    println!("{}", part2);
}

mod test {

    #[test]
    fn test_part1() {
        use super::check_part1;

        assert!(check_part1(111111));
        assert!(!check_part1(223450));
        assert!(!check_part1(123789));
    }

    #[test]
    fn test_part2() {
        use super::check_part2;

        assert!(check_part2(112233));
        assert!(!check_part2(123444));
        assert!(check_part2(111122));
    }
}
