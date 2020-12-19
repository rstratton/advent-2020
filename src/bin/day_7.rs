use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BagType(String);

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BagMember {
    quantity: u64,
    bag: BagType,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct BagRule {
    container: BagType,
    members: Vec<BagMember>,
}

struct BagGraph<'a> {
    bag_to_rule: HashMap<&'a BagType, &'a BagRule>,
}

impl<'a> BagGraph<'a> {
    fn new(rules: &'a [BagRule]) -> BagGraph<'a> {
        let mut bag_to_rule = HashMap::new();

        for rule in rules {
            bag_to_rule.insert(&rule.container, rule);
        }

        BagGraph { bag_to_rule }
    }

    fn can_contain(&self, src: &BagType, dst: &BagType) -> bool {
        let src_rule = self.bag_to_rule.get(src).unwrap();
        let mut bags_to_try: Vec<&BagType> = src_rule.members.iter().map(|m| &m.bag).collect();

        while let Some(bag_to_try) = bags_to_try.pop() {
            if bag_to_try == dst {
                return true;
            }

            let bag_rule = self.bag_to_rule.get(bag_to_try).unwrap();

            for member in &bag_rule.members {
                bags_to_try.push(&member.bag)
            }
        }

        false
    }

    fn count_recursive_members(&self, container: &BagType) -> u64 {
        let container_rule = self.bag_to_rule.get(container).unwrap();

        if container_rule.members.is_empty() {
            0
        } else {
            container_rule
                .members
                .iter()
                .map(|m| m.quantity * (1 + self.count_recursive_members(&m.bag)))
                .sum::<u64>()
        }
    }
}

peg::parser! {
    grammar bag_rule_parser() for str {
        pub rule bag_rule() -> BagRule
            = container:bag_type() " bags contain " members:bag_members() { BagRule { container, members } }

        rule bag_type() -> BagType
            = s:$(t:word() " " t:word()) { BagType(s.into()) }

        rule word() -> &'input str
            = $(['a'..='z']+)

        rule bag_members() -> Vec<BagMember>
            = no_members()
            / b:members_list()

        rule no_members() -> Vec<BagMember>
            = "no other bags." { Default::default() }

        rule members_list() -> Vec<BagMember>
            = m:member() ** ", " "."  { m }

        rule member() -> BagMember
            = n:$(['0'..='9']+) " " t:bag_type() " bag" "s"? { BagMember { quantity: n.parse().unwrap(), bag: t } }
    }
}

fn main() {
    let bag_rules: Vec<BagRule> = include_str!("../../data/day_7.txt")
        .lines()
        .flat_map(bag_rule_parser::bag_rule)
        .collect();

    let bag_graph = BagGraph::new(&bag_rules);

    // Part 1
    let shiny_gold_containers = bag_rules
        .iter()
        .filter(|r| bag_graph.can_contain(&r.container, &BagType("shiny gold".into())))
        .count();
    println!("{}", shiny_gold_containers);

    // Part 2
    let recursive_members_count = bag_graph.count_recursive_members(&BagType("shiny gold".into()));
    println!("{}", recursive_members_count)
}
