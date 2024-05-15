use std::process::{Command, Output};

fn execute_command (command: &mut Command, description: &str) -> Output {
  let output = command.output().expect(format!("Faile to run command: {}", description).as_str());
  if !output.status.success() {
    println!("Error: {}\nstdout: {}\nstderr: {}",
      description,
      String::from_utf8_lossy(&output.stdout),
      String::from_utf8_lossy(&output.stderr)
    );
  }
  output
}

pub fn commit_and_push(message: &str, path: &str) {
  execute_command(
    Command::new("git").arg("add").arg(path), 
    format!("git add {}", path).as_str()
  );

  execute_command(
    Command::new("git").arg("commit").arg("-m").arg(format!("\"{}\"", message)), 
    format!("git -m \"{}\"", message).as_str()
  );
  
  execute_command(
    Command::new("git").arg("push"), 
    format!("git push").as_str()
  );
}