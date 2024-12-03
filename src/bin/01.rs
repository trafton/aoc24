use std::collections::HashMap;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    
    let mut fs:Vec<u32> = Vec::new();
    let mut ls:Vec<u32> = Vec::new();
    for line in input.lines() {
        let vals = line.split_whitespace().take(2).collect::<Vec<&str>>();
        
        fs.push(vals.first().unwrap().parse().unwrap());
        ls.push(vals.last().unwrap().parse().unwrap());
    }

    fs.sort();
    ls.sort();
    let mut ans = 0;
    for (x,y) in fs.iter().zip(ls.iter()) {
        if x >y {
            ans += x - y;
        } else {
            ans += y -x;
        }
    }
    Some(ans)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut fs:Vec<u32> = Vec::new();
    let mut ls:Vec<u32> = Vec::new();
    for line in input.lines() {
        let vals = line.split_whitespace().take(2).collect::<Vec<&str>>();

        fs.push(vals.first().unwrap().parse().unwrap());
        ls.push(vals.last().unwrap().parse().unwrap());
    }
    
    let mut freq: HashMap<u32, u32> = HashMap::new();
    for i in ls {
        *freq.entry(i).or_default() += 1;
    }
    let mut ans = 0;
    for j in fs {
        match freq.get(&(j)) {
            None => {}
            Some(val) => {ans += (j * val);}
        }
    }
    
    Some(ans as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        println!("result: {:?}", result);
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
