use std::collections::HashSet;
use std::fmt;
use std::fs;

#[derive(Clone, Copy, Hash, Eq, PartialEq, Debug)]
struct P(isize, isize);

impl P {
    fn adjacent(&self) -> Vec<P> {
        vec![
            P(self.0, self.1 + 1),
            P(self.0, self.1 - 1),
            P(self.0 + 1, self.1),
            P(self.0 - 1, self.1),
        ]
    }
}

struct Layout {
    size: (usize, usize),
    bugs: HashSet<P>,
}

impl Layout {
    fn iterate(&mut self) {
        let mut bugs = HashSet::new();

        for x in 0..self.size.0 {
            for y in 0..self.size.1 {
                let p = P(x as isize, y as isize);

                let adjacent = p
                    .adjacent()
                    .iter()
                    .filter(|x| self.bugs.contains(x))
                    .count();

                if (self.bugs.contains(&p) && adjacent == 1)
                    || (!self.bugs.contains(&p) && (adjacent == 1 || adjacent == 2))
                {
                    bugs.insert(p);
                }
            }
        }

        self.bugs = bugs;
    }

    fn biodiversity(&self) -> usize {
        self.bugs
            .iter()
            .map(|p| 2_u32.pow(p.1 as u32 * self.size.0 as u32 + p.0 as u32) as usize)
            .sum()
    }
}

impl From<String> for Layout {
    fn from(input: String) -> Self {
        let mut bugs = HashSet::new();

        for (y, line) in input.lines().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char == '#' {
                    bugs.insert(P(x as isize, y as isize));
                }
            }
        }
        let height = input.lines().count();
        let width = input.lines().next().unwrap().chars().count();

        Layout {
            size: (width, height),
            bugs: bugs,
        }
    }
}

impl fmt::Display for Layout {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..self.size.1 {
            writeln!(
                f,
                "{}",
                (0..self.size.0)
                    .map(|x| if self.bugs.contains(&P(x as isize, y as isize)) {
                        '#'.to_string()
                    } else {
                        '.'.to_string()
                    })
                    .collect::<Vec<String>>()
                    .join("")
            )?
        }
        Ok(())
    }
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day24.txt").unwrap();
    let mut layout = Layout::from(input);

    let mut seen = HashSet::new();
    while !seen.contains(&layout.biodiversity()) {
        seen.insert(layout.biodiversity());
        layout.iterate();
    }
    println!("{}", layout);

    println!("{}", layout.biodiversity());
}

#[cfg(test)]
mod test {
    #[test]
    fn test_iterate() {
        use super::Layout;

        let mut layout = Layout::from(
            "....#
#..#.
#..##
..#..
#...."
                .to_string(),
        );
        println!("{}", layout);
        let next = Layout::from(
            "#..#.
####.
###.#
##.##
.##.."
                .to_string(),
        );
        layout.iterate();
        println!("{}", layout);

        assert_eq!(layout.bugs, next.bugs);
    }
}
