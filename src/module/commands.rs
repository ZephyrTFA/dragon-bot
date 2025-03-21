#[derive(Debug)]
pub enum CommandError {
    Serenity(serenity::Error),
}
