use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::process::Command;

use walkdir::WalkDir;

/// 搜索 testcase 文件夹下所有 `.lox` 文件
fn find_test_cases() -> Vec<(PathBuf, PathBuf)> {
    let mut cases = vec![];

    for entry in WalkDir::new("testcases")
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "lox"))
    {
        let input = entry.into_path();
        let output = input.with_extension("txt");
        cases.push((input, output));
    }

    cases
}


fn run_and_capture(path: &str) -> Result<String, String> {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--", path])
        .output()
        .expect("Failed to run test");

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}


#[test]
fn lox_test() {
    let cases = find_test_cases()
        .into_iter()
        .map(|(input, output)| (input.to_string_lossy().into_owned(), output.to_string_lossy().into_owned()))
        .collect::<Vec<_>>();
    eprintln!("Testing {} tests!", cases.len());
    for (input_path, expected_path) in cases {
        eprintln!("Testing `{}`", input_path);
        match run_and_capture(&input_path) {
            Ok(stdout) => {
                let normalize = |s: &str| s.trim_end().replace("\r\n", "\n");
                let mut expected = String::new();
                fs::File::open(&expected_path)
                    .and_then(|mut file| file.read_to_string(&mut expected))
                    .expect("Failed to read expected output");
                assert_eq!(
                    normalize(&stdout),
                    normalize(&expected),
                    "Mismatch for input file: {}",
                    input_path
                );
            }
            Err(err) => {
                eprintln!("Error running test: {}", err);
                assert!(false, "Test failed for input file: {}", input_path);
            }
        }
    }
    
}