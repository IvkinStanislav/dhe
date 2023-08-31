use std::collections::HashMap;

/// Get Intervals Between Identical Elements
/// Input: [2,1,3,1,2,3,3]
/// Output: [4,2,7,2,4,4,5]
pub fn get_distances(arr: Vec<i32>) -> Vec<i64> {
    let mut result = vec![0; arr.len()];
    let mut counter = HashMap::new();

    for (index, &value) in arr.iter().enumerate() {
        counter
            .entry(value)
            .and_modify(|(len, sum): &mut (usize, usize)| {
                result[index] += (*len * index - *sum) as i64;
                *len += 1;
                *sum += index;
            })
            .or_insert((1, index));
    }

    counter.clear();
    for (index, &value) in arr.iter().enumerate().rev() {
        counter
            .entry(value)
            .and_modify(|(len, sum): &mut (usize, usize)| {
                result[index] += (*len * index - *sum) as i64;
                *len += 1;
                *sum += index;
            })
            .or_insert((1, index));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_distances_1() {
        let input = vec![2, 1, 3, 1, 2, 3, 3];
        let output = vec![4, 2, 7, 2, 4, 4, 5];
        assert_eq!(output, get_distances(input));
    }

    #[test]
    fn get_distances_2() {
        let input = vec![10, 5, 10, 10];
        let output = vec![5, 0, 3, 4];
        assert_eq!(output, get_distances(input));
    }
}
