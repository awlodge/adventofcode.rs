use crate::helpers::dag::DirectedAcyclicGraph;

const INPUT: &str = include_str!("input/day11.txt");

pub fn run() -> (u64, u64) {
    let tree: DirectedAcyclicGraph<&str> = parse(INPUT);
    (tree.count_paths_you_to_out() as u64, 0)
}

fn parse(input: &str) -> DirectedAcyclicGraph<&str> {
    let mut tree = DirectedAcyclicGraph::new();
    for line in input.split('\n') {
        let line = line.trim();
        let parts: Vec<&str> = line.split(':').collect();
        let val = parts[0];
        let children: Vec<&str> = parts[1].split(' ').filter(|x| !x.is_empty()).collect();
        tree.insert(val, children);
    }
    tree
}

trait Cables {
    fn count_paths_you_to_out(&self) -> usize;
}

impl Cables for DirectedAcyclicGraph<&str> {
    fn count_paths_you_to_out(&self) -> usize {
        self.count_paths(&"you", &"out")
    }
}

#[cfg(test)]
mod test {
    use crate::y2025::day11::{Cables, parse};

    const TEST_INPUT: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_parse() {
        let tree = parse(TEST_INPUT);
        assert!(
            tree.get_children(&"you")
                .is_some_and(|x| x.contains(&"bbb"))
        );
    }

    #[test]
    fn test_count_paths() {
        let tree = parse(TEST_INPUT);
        assert_eq!(5, tree.count_paths_you_to_out());
    }
}
