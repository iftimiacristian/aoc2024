use regex::Regex;
use common::ParseError;

pub fn parse_input(input: &str) -> Result<Vec<(i32, i32)>, ParseError> {
    let all_pairs: Result<Vec<Vec<(i32, i32)>>, ParseError> = input
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| extract_mul_pairs(line))
        .collect();

    all_pairs.map(|vec_of_vecs| vec_of_vecs.into_iter().flatten().collect())
}

fn extract_mul_pairs(text: &str) -> Result<Vec<(i32, i32)>, ParseError>{
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut pairs = Vec::new();

    for captures in re.captures_iter(text) {
        let n1 = captures[1].parse::<i32>().map_err(ParseError::NumberParse)?;
        let n2 = captures[2].parse::<i32>().map_err(ParseError::NumberParse)?;

        pairs.push((n1, n2));
    }

    Ok(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let input = "mul(1,2)\nmul(3,4)\n\nmul(5,6)";
        let expected = vec![(1, 2), (3, 4), (5, 6)];
        assert_eq!(parse_input(input).unwrap(), expected);
    }

    #[test]
    fn test_extract_mul_pairs() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let expected = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
        assert_eq!(extract_mul_pairs(input).unwrap(), expected);
    }
}

