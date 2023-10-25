use anyhow::Result;

fn get_acc_signal_strength(commands: &[String]) -> i32 {
    // Init variables
    let mut x_reg = 1;
    let mut clock_cycles = 0;
    let mut acc_signal_strength = 0;

    // Clock tick callback
    let mut on_tick = |x_reg: i32| {
        if (clock_cycles + 1) % 40 == 20 {
            acc_signal_strength += x_reg * (clock_cycles + 1);
        }
        clock_cycles += 1;
    };

    // Process commands
    commands.iter().for_each(|cmd| {
        on_tick(x_reg);
        if cmd != "noop" {
            let (_, val_str) = cmd.split_once(' ').unwrap();
            let val = val_str.parse::<i32>().unwrap();
            on_tick(x_reg);
            x_reg += val;
        }
    });
    acc_signal_strength
}

fn draw_display_image(commands: &[String]) -> String {
    // Init variables
    let mut x_reg = 1;
    let mut clock_cycles = 0;
    let mut display = String::new();

    // Clock tick callback
    let mut on_tick = |x_reg: i32| {
        let pos = clock_cycles % 40;
        if pos == 0 && clock_cycles != 0 {
            display += "\n";
        }
        let sprite_pos: [i32; 3] = [x_reg - 1, x_reg, x_reg + 1];
        if sprite_pos.contains(&pos) {
            display += "#";
        } else {
            display += ".";
        }
        clock_cycles += 1;
    };

    // Process commands
    commands.iter().for_each(|cmd| {
        on_tick(x_reg);
        if cmd != "noop" {
            let (_, val_str) = cmd.split_once(' ').unwrap();
            let val = val_str.parse::<i32>().unwrap();
            on_tick(x_reg);
            x_reg += val;
        }
    });
    display
}

fn main() -> Result<()> {
    let commands = std::fs::read_to_string("data/10.input")?
        .lines()
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    println!("Part 1: {}", get_acc_signal_strength(&commands));
    println!("Part 2:\n{}", draw_display_image(&commands));

    Ok(())
}

#[cfg(test)]
mod day10 {
    use super::*;

    #[test]
    fn part1() -> Result<()> {
        let commands = std::fs::read_to_string("data/10.test")?
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        assert_eq!(get_acc_signal_strength(&commands), 13140);
        Ok(())
    }

    #[test]
    fn part2() -> Result<()> {
        let commands = std::fs::read_to_string("data/10.test")?
            .lines()
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let res = "##..##..##..##..##..##..##..##..##..##..\n\
                   ###...###...###...###...###...###...###.\n\
                   ####....####....####....####....####....\n\
                   #####.....#####.....#####.....#####.....\n\
                   ######......######......######......####\n\
                   #######.......#######.......#######.....";
        assert_eq!(draw_display_image(&commands), res);
        Ok(())
    }
}
