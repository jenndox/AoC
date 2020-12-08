pub mod bags {
    pub use std::collections::HashMap;

    #[derive(Debug, Eq, Ord, PartialEq, PartialOrd, Clone)]
    struct Rule {
        bag_type: String,
        count: u64
    }

    impl Rule {
        pub fn new(bag_type: String, count: u64) -> Self {
            Rule {
                bag_type,
                count
            }
        }
    }

    pub use crate::helper::helper_fns;

    pub fn shiny(filename: &String) {
        const RADIX: u64 = 10;
        let data = helper_fns::file_into_vec(filename);

        let mut rules: HashMap<String, Vec<Rule>> = HashMap::new();

        for x in &data {
            // Parse the line to find the bag type and rules
            let rule_segs: Vec<_> = x.split("contain").collect();
            if rule_segs.len() > 0 {
                let main_type = rule_segs[0].trim();
                let main_name_parts: Vec<_> = main_type.split("bag").collect();
                let main_rule_trim = main_name_parts[0].trim().to_string();
                let mut curr_rules: Vec<Rule>;

                match rules.get(&main_rule_trim) {
                    //Some(&rules_set_curr) => curr_rules = rules_set_curr,
                    _ => curr_rules = Vec::new(),
                }

                let rule_set: Vec<_> = rule_segs[1].split(',').collect();
                for rule in &rule_set {
                    let mut rule_chars: Vec<char> = rule.chars().collect();
                    let count = rule_chars[1];
                    rule_chars.remove(1);
                    rule_chars.remove(rule_chars.len() - 1);
                    let rule_data: Rule;
                    match count.to_digit(RADIX as u32)  {
                        Some(rule_count) =>  {
                            let rule_name = rule_chars.into_iter().collect::<String>();
                            let name_parts: Vec<_> = rule_name.split("bag").collect();
                            let rule_trim = name_parts[0].trim().to_string();
                            rule_data = Rule::new(rule_trim, rule_count.into())
                        },
                        _ => rule_data = Rule::new(rule_chars.into_iter().collect::<String>().trim().to_string(), 0),
                    }
                    curr_rules.push(rule_data);
                }
                rules.insert(main_rule_trim.to_string(), curr_rules);
            }
        }

        let bags_needed = find_bags_for_color(&rules, &"shiny gold".to_string(), 1);
        println!("Current bags needed total: {}", bags_needed - 1);
    }
       

    fn _part_one() { 
        //let mut candidates = find_rules_by_color(&rules, &"shiny gold".to_string());
        //candidates.sort();
        //candidates.dedup();
        //println!("Current candidates total: {}", candidates.len());
    }

    fn find_bags_for_color(rules: &HashMap<String, Vec<Rule>>, color_name: &String, mut bag_ct: u64) -> u64 {
        let rules_for_color = rules.get(color_name);
        let mut new_bags_for_color = 0;
        match rules_for_color {
            Some(rules_group) => {
                let rules_vec: Vec<Rule> = rules_group.to_vec();
                for rule in rules_vec {
                    println!("Rule found for {} of {} bags in {} type.", rule.count, color_name, rule.bag_type);
                    let bags_for_color =  find_bags_for_color(&rules, &rule.bag_type, bag_ct);
                    new_bags_for_color += rule.count * bags_for_color;
                    println!("Current bags needed: {}", new_bags_for_color);
                }
            },
            _ => {}
        }
        println!("need {} bags for color {}", new_bags_for_color, color_name);
        bag_ct += new_bags_for_color;
        bag_ct
    }

    fn _find_rules_by_color(rules: &HashMap<String, Vec<Rule>>, color_name: &String) -> Vec<String>{
        let mut candidates: Vec<String> = Vec::new();
        for (bag_color, rule_group) in rules {
            for rule in rule_group {
                if rule.bag_type.contains(color_name) {
                    println!("Can put {} in bags of type {}", color_name, bag_color);
                    let seg_name: Vec<_> = bag_color.split("bag").collect();
                    let cleaned_name = seg_name[0];
                    candidates.push(cleaned_name.to_string());

                    candidates.extend(_find_rules_by_color(&rules, &cleaned_name.to_string()));
                }
            }
        }
        candidates
    }
}
