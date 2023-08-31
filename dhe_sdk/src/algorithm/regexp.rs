use std::collections::HashSet;

/// Regular expression with support for '.' and '*'
pub fn is_match(input: String, pattern: String) -> bool {
    let mut state_machine = HashSet::new();
    let mut graph: Vec<(char, Vec<usize>)> = vec![];
    graph.push((0 as char, vec![1]));

    let mut jumpers = vec![0];

    let pattern: Vec<_> = pattern.chars().collect();
    for i in 0..pattern.len() {
        let current = pattern[i];
        let next = pattern.get(i + 1).copied();
        if current != '*' {
            let current_state = graph.len();
            let next_state = current_state + 1;
            let mut refs = vec![next_state];

            if matches!(next, Some(next) if next == '*') {
                refs.push(current_state);

                for &jumper in &jumpers {
                    if let Some(no_star) = graph.get_mut(jumper) {
                        no_star.1.push(next_state);
                    }
                }
            } else {
                jumpers.clear();
            }

            jumpers.push(current_state);
            graph.push((current, refs));
        }
    }

    if let Some(first_vertex) = graph.first() {
        for &r in &first_vertex.1 {
            state_machine.insert(r);
        }
    }

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

    #[test]
    fn is_match_6() {
        let input = "aaa".to_owned();
        let pattern = "ab*a*c*a".to_owned();
        assert!(is_match(input, pattern));
    }
}
