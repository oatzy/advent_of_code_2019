use std::fs;
use std::iter;

#[derive(Clone, Copy)]
enum Shuffle {
    IntoNewStack,
    WithIncrement(usize),
    Cut(isize),
}

struct Deck {
    cards: Vec<usize>,
}

impl Deck {
    fn new(size: usize) -> Self {
        Deck {
            cards: (0..size).collect(),
        }
    }

    fn deal_into_new_stack(&mut self) {
        self.cards.reverse();
    }

    fn deal_with_increment(&mut self, size: usize) {
        let length = self.cards.len();
        let mut shuffled: Vec<usize> = iter::repeat(0).take(length).collect();

        for (i, n) in self.cards.iter().enumerate() {
            shuffled[(i * size) % length] = *n;
        }

        self.cards = shuffled;
    }

    fn cut(&mut self, size: isize) {
        if size > 0 {
            self.cards.rotate_left(size as usize)
        } else {
            self.cards.rotate_right(size.abs() as usize)
        }
    }

    fn shuffle(&mut self, steps: &Vec<Shuffle>) {
        for step in steps {
            match step {
                Shuffle::IntoNewStack => self.deal_into_new_stack(),
                Shuffle::Cut(size) => self.cut(*size),
                Shuffle::WithIncrement(size) => self.deal_with_increment(*size),
            }
        }
    }
}

fn position_after_shuffles(position: usize, card_count: usize, shuffle: &Vec<Shuffle>) -> usize {
    let mut p = position;
    for s in shuffle {
        p = match s {
            Shuffle::IntoNewStack => card_count - position - 1,
            Shuffle::Cut(size) if *size < 0 => (position + size.abs() as usize) % card_count,
            Shuffle::Cut(size) => (position + card_count - *size as usize) % card_count,
            Shuffle::WithIncrement(size) => (position * size) % card_count,
        }
    }
    p
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day22.txt").unwrap();

    let steps: Vec<Shuffle> = input
        .lines()
        .map(|line| {
            if line.contains("new") {
                Shuffle::IntoNewStack
            } else if line.contains("cut") {
                Shuffle::Cut(line.split_whitespace().last().unwrap().parse().unwrap())
            } else if line.contains("increment") {
                Shuffle::WithIncrement(line.split_whitespace().last().unwrap().parse().unwrap())
            } else {
                panic!("unexpected line {}", line)
            }
        })
        .collect();

    // let mut deck = Deck::new(10007);
    // deck.shuffle(&steps);

    // println!("{}", deck.cards.iter().position(|&x| x == 2019).unwrap());
    println!("{}", position_after_shuffles(2019, 10007, &steps));

    // let part2 = (0..119315717514047)
    //     .map(|pos| {
    //         let mut p = pos;
    //         for _ in 0..101741582076661_u64 {
    //             p = position_after_shuffles(p, 119315717514047, &steps);
    //         }
    //         p
    //     })
    //     .position(|x| x == 2020)
    //     .unwrap();

    //println!("{}", part2);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_deal_into_stack() {
        use super::Deck;

        let mut deck = Deck::new(10);
        deck.deal_into_new_stack();
        assert_eq!(deck.cards, (0..10).rev().collect::<Vec<usize>>());
    }

    #[test]
    fn test_deal_with_increment() {
        use super::Deck;

        let mut deck = Deck::new(10);
        deck.deal_with_increment(3);

        assert_eq!(deck.cards, vec![0, 7, 4, 1, 8, 5, 2, 9, 6, 3])
    }

    #[test]
    fn test_cut() {
        use super::Deck;

        let mut deck = Deck::new(10);
        deck.cut(3);

        assert_eq!(deck.cards, vec![3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);
    }

    #[test]
    fn test_cut_negative() {
        use super::Deck;

        let mut deck = Deck::new(10);
        deck.cut(-4);

        assert_eq!(deck.cards, vec![6, 7, 8, 9, 0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn test_example1() {
        use super::Deck;

        let mut deck = Deck::new(10);
        deck.deal_with_increment(7);
        deck.deal_into_new_stack();
        deck.deal_into_new_stack();

        assert_eq!(deck.cards, vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7]);
    }

    #[test]
    fn test_example2() {
        use super::Deck;

        let mut deck = Deck::new(10);
        deck.cut(6);
        deck.deal_with_increment(7);
        deck.deal_into_new_stack();

        assert_eq!(deck.cards, vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6]);
    }

    #[test]
    fn test_example3() {
        use super::Deck;

        let mut deck = Deck::new(10);
        deck.deal_with_increment(7);
        deck.deal_with_increment(9);
        deck.cut(-2);

        assert_eq!(deck.cards, vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9]);
    }

    #[test]
    fn test_example4() {
        use super::Deck;

        let mut deck = Deck::new(10);
        deck.deal_into_new_stack();
        deck.cut(-2);
        deck.deal_with_increment(7);
        deck.cut(8);
        deck.cut(-4);
        deck.deal_with_increment(7);
        deck.cut(3);
        deck.deal_with_increment(9);
        deck.deal_with_increment(3);
        deck.cut(-1);

        assert_eq!(deck.cards, vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6]);
    }
}
