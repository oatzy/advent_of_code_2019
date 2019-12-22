use std::fs;
use std::iter;

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
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day22.txt").unwrap();

    let mut deck = Deck::new(10007);

    for line in input.lines() {
        if line.contains("new") {
            deck.deal_into_new_stack();
        } else if line.contains("cut") {
            deck.cut(line.split_whitespace().last().unwrap().parse().unwrap());
        } else if line.contains("increment") {
            deck.deal_with_increment(line.split_whitespace().last().unwrap().parse().unwrap());
        } else {
            panic!("unexpected line {}", line)
        }
    }

    println!("{}", deck.cards.iter().position(|&x| x == 2019).unwrap());
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
