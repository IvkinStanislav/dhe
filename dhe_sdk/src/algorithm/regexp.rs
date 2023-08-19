/// Regular expression with support for '.' and '*'
pub fn is_match(input: String, pattern: String) -> bool {
    let mut state_indexes = vec![0]; // TODO to set
    let mut pattern_state: Vec<(char, Vec<usize>)> = vec![];

    for ch in pattern.chars() {
        if ch == '*' {
            let next_num = pattern_state.len();
            let current_num = next_num - 1;
            let prev_num = current_num.checked_sub(1);

            if let Some(last) = pattern_state.last_mut() {
                (*last).1.push(current_num);
            }

            if let Some(prev_num) = prev_num {
                let prev = unsafe{ pattern_state.get_unchecked_mut(prev_num) };
                (*prev).1.push(next_num);
            }

        } else {
            let next_num = pattern_state.len() + 1;
            pattern_state.push((ch, vec![next_num]));
        }
    }
    println!("state_indexes: {state_indexes:?}");
    println!("pattern_state: {pattern_state:?}");

    for ch in input.chars() {
        while let Some(state_index) = state_indexes.pop() {
            if let Some(state) = pattern_state.get(state_index) {
                if state.0 == ch || state.0 == '.' {
                    state_indexes.extend_from_slice(&state.1);
                    break;
                } else if !state_indexes.is_empty() {
                    continue;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }
    }

    true
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
}
