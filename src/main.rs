use std::process::Command;
use std::fs::File;
use std::io::Write;
use rand::Rng;
use chrono::Utc;

fn main() {
    let x = 1; // Minimum number of commits
    let y = 5; // Maximum number of commits

    let commit_count = rand::thread_rng().gen_range(x..=y);

    for i in 1..=commit_count {
        let file_name = format!("dummy_file_{}.txt", i);
        let mut file = File::create(&file_name).expect("Failed to create file");
        writeln!(file, "This is a dummy commit created at {}", Utc::now()).expect("Failed to write to file");

        Command::new("git")
            .args(["add", &file_name])
            .output()
            .expect("Failed to stage file");

        Command::new("git")
            .args(["commit", "-m", &format!("Random commit {}", i)])
            .output()
            .expect("Failed to commit");
    }

    Command::new("git")
        .args(["push"])
        .output()
        .expect("Failed to push commits");

    println!("Pushed {} random commits to the repository!", commit_count);
}
