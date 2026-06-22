use crate::manifest::{Launch, Manifest};

pub fn manifest(game_name: &str) -> Manifest {
    Manifest {
        name: game_name.to_string(),
        runner: "whisky".to_string(),
        dependencies: Vec::new(),
        launch: Launch {
            executable: String::new(),
        },
    }
}