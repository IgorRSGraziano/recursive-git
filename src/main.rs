fn main() {
    let args: Vec<String> = std::env::args().collect();

    fn recursive_execute(args: &Vec<String>, path: &std::path::PathBuf) {
        if path.join(".git").exists() {
            let mut command = std::process::Command::new("git");

            let mut git_args: Vec<&str> = vec!["-c", "color.status=always"];
            let mut proxy_args = args[1..].iter().map(|s| s.as_str()).collect::<Vec<_>>();
            git_args.append(&mut proxy_args);
            command.args(git_args);
            command.current_dir(path);
            let output = command.output().expect("Failed to execute command");
            println!("{}", String::from_utf8_lossy(&output.stdout));
            println!("{}", String::from_utf8_lossy(&output.stderr));
        }

        if path.is_dir() {
            for entry in std::fs::read_dir(path).expect("Failed to read directory") {
                let entry = entry.expect("Failed to read entry");
                let path = entry.path();
                recursive_execute(args, &path);
            }
        }
    }

    recursive_execute(
        &args,
        &std::env::current_dir().expect("Failed to get current directory"),
    );
}
