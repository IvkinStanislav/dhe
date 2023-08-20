use std::collections::HashSet;

/// Regular expression with support for '.' and '*'
pub fn is_match(input: String, pattern: String) -> bool {
    let mut state_machine = HashSet::new();
    state_machine.insert(0);
    let mut graph: Vec<(char, Vec<usize>)> = vec![];

    for ch in pattern.chars() {
        if ch == '*' {
            let next_num = graph.len();
            let current_num = next_num - 1;
            let prev_num = current_num.checked_sub(1);

            if let Some(last) = graph.last_mut() {
                (*last).1.push(current_num);
            }

            if let Some(prev_num) = prev_num {
                let prev = unsafe{ graph.get_unchecked_mut(prev_num) };
                (*prev).1.push(next_num);
            }

            if state_machine.contains(&current_num) {
                state_machine.insert(next_num);
            }
        } else {
            let next_num = graph.len() + 1;
            graph.push((ch, vec![next_num]));
        }
    }
    println!("state_machine: {state_machine:?}");
    println!("graph: {graph:?}");

    for ch in input.chars() {
        let mut new_state_machine = HashSet::new();
        for &state in &state_machine {
            if let Some(vertex) = graph.get(state) {
                if vertex.0 != ch && vertex.0 != '.' {
                    continue;
                }
                for &next_state in &vertex.1 {
                    new_state_machine.insert(next_state);
                }
            }
        }
        state_machine.clear();
        state_machine = new_state_machine;
    }
    println!("state_machine_after`: {state_machine:?}");
    state_machine.contains(&graph.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_match_1() {
        let input = "aa".to_owned();
        let pattern = "a".to_owned();
        assert!(!is_match(input, pattern));
    }

    #[test]
    fn is_match_2() {
        let input = "aa".to_owned();
        let pattern = "a*".to_owned();
        assert!(is_match(input, pattern));
    }

    #[test]
    fn is_match_3() {
        let input = "ab".to_owned();
        let pattern = ".*".to_owned();
        assert!(is_match(input, pattern));
    }

    #[test]
    fn is_match_4() {
        let input = "aab".to_owned();
        let pattern = "c*a*b".to_owned();
        assert!(is_match(input, pattern));
    }

    #[test]
    fn is_match_5() {
        let input = "baabbbaccbccacacc".to_owned();
        let pattern = "c*..b*a*a.*a..*c".to_owned();
        assert!(is_match(input, pattern));
    }
}
