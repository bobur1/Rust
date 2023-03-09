mod player;

fn main() {
    player::play_movie("cartoon.mp4");
    player::play_audio("boom.mp3");

    clean::perform_clean();
    clean::files::clean_files();
}

mod clean {
    pub fn perform_clean() {
        println!("Cleaning ssd");
    }

    pub mod files {
        pub fn clean_files() {
            println!("Removing unused files");
        }
    }
}