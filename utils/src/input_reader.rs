use anyhow::Result;
use anyhow::anyhow;
use std::path::PathBuf;

#[macro_export]
macro_rules! load_test_input {
    () => {
        $crate::input_reader::load_file_content(env!("CARGO_MANIFEST_DIR"), std::file!(), None)
            .unwrap();
    };
    ($part:expr) => {
        $crate::input_reader::load_file_content(
            env!("CARGO_MANIFEST_DIR"),
            std::file!(),
            Some($part),
        )
        .unwrap();
    };
}

pub fn load_file_content(
    crate_path: &str,
    solution_path: &str,
    part: Option<u32>,
) -> Result<String> {
    let crate_path = PathBuf::from(crate_path);
    let solution_path = PathBuf::from(solution_path);

    let file_stem = solution_path
        .file_stem()
        .ok_or(anyhow!("Could not get file stem."))?
        .to_str()
        .ok_or(anyhow!("Could not convert file name to string."))?;

    let day = file_stem[3..5].parse::<u32>().map_err(|_| {
        anyhow!("The expected solution file should be in the form `dayXX.rs`. Got: {file_stem}")
    })?;

    if part.is_some_and(|p| ![1, 2].contains(&p)) {
        return Err(anyhow!("The part {} is invalid.", part.unwrap()));
    }

    let test_file_name = if let Some(part) = part {
        format!("{:02}-{part}.txt", day)
    } else {
        format!("{:02}.txt", day)
    };

    let test_input_path = crate_path
        .join("resources")
        .join("tests")
        .join(test_file_name);

    if !test_input_path.exists() {
        return Err(anyhow!(
            "The file {} does not exist.",
            test_input_path.display()
        ));
    }

    Ok(std::fs::read_to_string(&test_input_path)?)
}
