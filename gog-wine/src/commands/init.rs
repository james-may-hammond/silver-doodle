pub fn run(game_name: String, directory: Option<String>) -> anyhow::Result<()> {
    println!("Game: {}", game_name);

    if let Some(dir) = directory {
        println!("Directory: {}", dir);
    }

    Ok(())
}
