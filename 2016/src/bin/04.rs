use std::collections::BTreeMap;

#[derive(Debug)]
struct RoomData<'a> {
    name: &'a str,
    sector_id: u32,
    checksum: &'a str,
}

impl<'a> RoomData<'a> {
    fn new(input: &'a str) -> anyhow::Result<Self> {
        let input = input.trim_end_matches(']');
        let checksum_pos = input.rfind('[').ok_or(anyhow::anyhow!("Invalid input"))?;
        let name = &input[..checksum_pos - 3];
        let sector_id = input[checksum_pos - 3..checksum_pos].parse::<u32>()?;
        let checksum = &input[checksum_pos + 1..];

        Ok(Self {
            name,
            sector_id,
            checksum,
        })
    }

    fn is_valid(&self) -> bool {
        let mut counts = BTreeMap::new();
        for c in self.name.chars() {
            if c == '-' {
                continue;
            }
            *counts.entry(c).or_insert(0) += 1;
        }
        let mut counts: Vec<_> = counts.into_iter().collect();
        // Sort by count, then by alphabet
        counts.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
        let checksum: String = counts.iter().take(5).map(|x| x.0).collect();
        checksum == self.checksum
    }

    fn decrypt(&self) -> String {
        let mut result = String::new();
        for c in self.name.chars() {
            if c == '-' {
                result.push(' ');
                continue;
            }
            let c = c as u32 - b'a' as u32;
            let c = (c + self.sector_id) % 26;
            result.push((c as u8 + b'a') as char);
        }
        result
    }
}

fn main() -> anyhow::Result<()> {
    let input: String = std::fs::read_to_string("data/04.input")?;
    let rooms = input
        .lines()
        .map(RoomData::new)
        .filter_map(Result::ok)
        .collect::<Vec<_>>();

    let valid_rooms: usize = rooms
        .iter()
        .filter(|x| x.is_valid())
        .map(|x| x.sector_id as usize)
        .sum();

    println!("Part 1: {valid_rooms}");

    let north_pole_sector_id = rooms
        .iter()
        .find(|x| x.decrypt().contains("northpole object storage"))
        .ok_or(anyhow::anyhow!("Not found"))?
        .sector_id;

    println!("Part 2: {north_pole_sector_id}");

    Ok(())
}

#[cfg(test)]
mod day04 {
    use super::*;

    #[test]
    fn part1() -> anyhow::Result<()> {
        assert!(RoomData::new("aaaaa-bbb-z-y-x-123[abxyz]")?.is_valid());
        assert!(RoomData::new("a-b-c-d-e-f-g-h-987[abcde]")?.is_valid());
        assert!(RoomData::new("not-a-real-room-404[oarel]")?.is_valid());
        assert!(!RoomData::new("totally-real-room-200[decoy]")?.is_valid());
        Ok(())
    }

    #[test]
    fn part2() -> anyhow::Result<()> {
        assert_eq!(
            RoomData::new("qzmt-zixmtkozy-ivhz-343[abcde]")?
                .decrypt()
                .trim_end(),
            "very encrypted name"
        );
        Ok(())
    }
}
