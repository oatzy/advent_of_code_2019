use std::collections::HashMap;
use std::fs;

#[derive(Debug, Clone)]
struct Reaction {
    inputs: HashMap<String, usize>,
    output: String,
    amount: usize,
}

impl Reaction {
    fn required_for(&self, amount: usize) -> usize {
        if self.amount >= amount {
            self.amount
        } else {
            self.amount * (amount as f64 / self.amount as f64).ceil() as usize
        }
    }

    fn request(&self, amount: usize) -> HashMap<String, usize> {
        if self.amount >= amount {
            self.inputs.clone()
        } else {
            let multiplier = (amount as f64 / self.amount as f64).ceil() as usize;
            let mut result = HashMap::new();
            for (input, amount) in self.inputs.iter() {
                result.insert(input.clone(), multiplier * amount);
            }
            result
        }
    }
}

impl From<String> for Reaction {
    fn from(line: String) -> Self {
        let mut inout = line.trim().split(" => ");
        let mut inputs = HashMap::new();
        for mut input in inout.next().unwrap().split(", ").map(|x| x.split(" ")) {
            let count = input.next().unwrap().parse().unwrap();
            let id = input.next().unwrap();
            inputs.insert(id.to_owned(), count);
        }
        let mut output = inout.next().unwrap().split(" ");
        let count = output.next().unwrap().parse().unwrap();
        let id = output.next().unwrap();
        Reaction {
            inputs: inputs,
            output: id.to_owned(),
            amount: count,
        }
    }
}

#[derive(Debug, Clone)]
struct Reactions {
    outputs: HashMap<String, Reaction>,
}

impl Reactions {
    fn set_fuel(&mut self, fuel: usize) {
        let mut reaction = self.outputs.remove("FUEL").unwrap();
        reaction.amount = fuel;
        for (_, amount) in reaction.inputs.iter_mut() {
            *amount *= fuel;
        }
        self.outputs.insert("FUEL".to_string(), reaction);
    }
}

impl From<String> for Reactions {
    fn from(lines: String) -> Self {
        let mut outputs = HashMap::new();
        for line in lines.lines() {
            let reaction = Reaction::from(line.to_string());
            outputs.insert(reaction.output.clone(), reaction);
        }
        Reactions { outputs: outputs }
    }
}

fn fuel_requirements(reactions: &Reactions) -> usize {
    let mut required = reactions.outputs.get("FUEL").unwrap().inputs.clone();
    let mut spares = HashMap::new();
    // println!("{:?}", required);
    let mut ore = 0;

    while required.len() != 0 {
        let mut updated = HashMap::new();
        // println!("{:?}", required);
        for (item, &amount) in required.iter() {
            // println!("{}: {}", item, amount);
            // println!("{:?}", spares);
            let spare = spares.entry(item.to_string()).or_insert(0);
            if *spare >= amount {
                *spare -= amount;
            } else if item == "ORE" {
                ore += amount
            } else {
                let reaction = reactions.outputs.get(item).unwrap();
                *spare += reaction.required_for(amount) - amount;
                let request = reaction.request(amount);
                // println!("{:?}", request);

                for input in reaction.inputs.keys() {
                    let available = request.get(input).unwrap();
                    *updated.entry(input.to_string()).or_insert(0) += available;
                }
            }
        }
        required = updated;
    }

    // correct for any fuel we don't need for spare material
    loop {
        let mut updated = HashMap::new();
        for (id, &count) in spares.iter() {
            if id == "ORE" {
                ore -= count;
            } else {
                let reaction = reactions.outputs.get(id).unwrap();
                if count >= reaction.amount {
                    let multiplier = (count as f64 / reaction.amount as f64).floor() as usize;
                    for (item, amount) in reaction.inputs.iter() {
                        *updated.entry(item.to_string()).or_insert(0) += amount * multiplier;
                    }
                    *updated.entry(id.to_string()).or_insert(0) +=
                        count - multiplier * reaction.amount;
                } else {
                    *updated.entry(id.to_string()).or_insert(0) += count;
                }
            }
        }
        if spares == updated {
            break;
        }
        spares = updated;
    }

    ore
}

fn find_max(reactions: &Reactions) -> usize {
    // result from part 1 = 504284
    // so the correct answer is going to be in the region of 1e12/part1
    // then I did manual interpolation to find the point
    let mut fuel = 2690790;

    loop {
        let mut test_reactions = reactions.clone();
        test_reactions.set_fuel(fuel);
        let ore = fuel_requirements(&test_reactions);
        println!("fuel = {}, ore = {}", fuel, ore);
        if ore > 1000000000000 {
            break;
        }
        fuel += 1;
    }
    fuel - 1
}

fn main() {
    let input = fs::read_to_string("/home/chris/advent_of_code/2019/inputs/day14.txt").unwrap();
    let reactions = Reactions::from(input);
    // let part1 = fuel_requirements(&reactions);
    // println!("{}", part1);

    let part2 = find_max(&reactions);
    println!("{}", part2);
}

#[cfg(test)]
mod test {

    #[test]
    fn test_basic1() {
        use super::{fuel_requirements, Reactions};

        let input = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL"
            .to_string();
        let reactions = Reactions::from(input);
        println!("{:?}", reactions);
        assert_eq!(fuel_requirements(&reactions), 31);
    }

    #[test]
    fn test_example2() {
        use super::{fuel_requirements, Reactions};

        let input = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL"
            .to_string();
        let reactions = Reactions::from(input);
        println!("{:?}", reactions);
        assert_eq!(fuel_requirements(&reactions), 165);
    }

    #[test]
    fn test_example3() {
        use super::{fuel_requirements, Reactions};

        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            .to_string();
        let reactions = Reactions::from(input);
        println!("{:?}", reactions);
        assert_eq!(fuel_requirements(&reactions), 13312);
    }

    #[test]
    fn test_basic1_part2() {
        use super::{fuel_requirements, Reactions};

        let input = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT"
            .to_string();
        let mut reactions = Reactions::from(input);
        let expect = 82892754;
        reactions.set_fuel(expect);
        println!("{:?}", reactions);
        let requirements = fuel_requirements(&reactions);
        println!("requirement = {}", requirements);
        assert!(requirements < 1000000000000);
    }
}
