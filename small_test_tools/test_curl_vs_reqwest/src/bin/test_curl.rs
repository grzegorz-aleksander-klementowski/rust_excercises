use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for i in 0..100 {
        let output = Command::new("curl")
            .arg("-s")
            .arg("http://example.com")
            .output()?;

        if output.status.success() {
            println!(
                "Iteration {}: {}",
                i + 1,
                String::from_utf8_lossy(&output.stdout)
            );
        } else {
            eprintln!(
                "Iteration {}: Error: {}",
                i + 1,
                String::from_utf8_lossy(&output.stderr)
            );
        }
    }
    Ok(())
}
