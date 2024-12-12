advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    let values: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .chain(Some(0))
        .collect();

    let mut v: Vec<i32> = Vec::with_capacity(values.iter().sum::<u32>() as usize);

    values.chunks(2).enumerate().for_each(|(id, x)| {
        v.extend(std::iter::repeat(id as i32).take(x[0] as usize));
        v.extend(std::iter::repeat(-1).take(x[1] as usize));
    });

    // move items
    let (mut i, mut j) = (0, v.len() - 1);

    while i < j {
        while v[i] >= 0 {
            i += 1;
        }

        while v[j] < 0 {
            j -= 1;
        }

        if i < j {
            v[i] = v[j];
            v[j] = -1;
        }
    }

    // calculate sum
    Some(calculate_checksum(&v))
}

fn calculate_checksum(v: &[i32]) -> i64 {
    v.iter()
        .enumerate()
        .take_while(|(_, file_id)| **file_id >= 0)
        .map(|(pos, file_id)| pos as i64 * *file_id as i64)
        .sum()
}

#[derive(Debug)]
struct Block {
    file_id: u32,
    start: u32,
    length: u32,
}

impl Block {
    fn checksum(&self) -> u32 {
        self.file_id * (self.start..(self.start + self.length)).sum::<u32>()
    }
}

#[derive(Debug)]
struct Gap {
    start: u32,
    length: u32,
}

pub fn part_two(input: &str) -> Option<u64> {
    // [0, 0, 2], [1, 5, 3], [2,
    // [2, 3]
    let values: Vec<u32> = input
        .chars()
        .map(|c| c.to_digit(10).unwrap())
        .chain(Some(0))
        .collect();

    let mut blocks: Vec<Block> = Vec::with_capacity(values.len() / 2);
    let mut gaps: Vec<Gap> = Vec::with_capacity(values.len() / 2);
    let mut start = 0;

    for (id, chunk) in values.chunks(2).enumerate() {
        blocks.push(Block {
            file_id: id as u32,
            start,
            length: chunk[0],
        });
        start += chunk[0];

        if chunk[1] > 0 {
            gaps.push(Gap {
                start,
                length: chunk[1],
            });
            start += chunk[1];
        }
    }

    blocks.iter_mut().rev().for_each(|block| {
        if let Some(x) = gaps
            .iter_mut()
            .find(|gap| gap.length >= block.length && gap.start < block.start)
        {
            block.start = x.start;

            x.length -= block.length;
            x.start += block.length;
        }
    });

    Some(blocks.iter().map(|b| b.checksum() as u64).sum::<u64>())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1928));
    }

    #[test]
    fn test_checksum() {
        let b = Block {
            file_id: 9,
            start: 2,
            length: 2,
        };

        assert_eq!(45, b.checksum());
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2858));
    }
}
