use anyhow::Result;

fn find_distinct_consecutive_chars(buffer: &str, n: usize) -> Result<usize> {
    if let Some(pos) = buffer
        .chars()
        .collect::<Vec<char>>()
        .windows(n)
        .position(|w| {
            let mut chars = w.to_vec();
            chars.sort();
            chars.dedup();
            chars.len() == n
        })
    {
        Ok(pos + n)
    } else {
        Err(anyhow::anyhow!("No distinct consecutive characters found"))
    }
}

fn main() -> Result<()> {
    let buffer = std::fs::read_to_string("./data/06.input")?;
    println!(
        "Start-of-packet marker: {}",
        find_distinct_consecutive_chars(&buffer, 4)?
    );
    println!(
        "Start-of-message marker: {}",
        find_distinct_consecutive_chars(&buffer, 14)?
    );
    Ok(())
}

#[cfg(test)]
mod day06 {
    use super::*;

    #[test]
    fn start_of_packet() -> Result<()> {
        let buffer = std::fs::read_to_string("./data/06.test")?;
        assert_eq!(find_distinct_consecutive_chars(&buffer, 4)?, 7);
        Ok(())
    }

    #[test]
    fn start_of_message() -> Result<()> {
        let buffer = std::fs::read_to_string("./data/06.test")?;
        assert_eq!(find_distinct_consecutive_chars(&buffer, 14)?, 19);
        Ok(())
    }
}
