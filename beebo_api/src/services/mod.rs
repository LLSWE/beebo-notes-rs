mod git_service;
mod send_question;
mod take_notes;

pub use git_service::send_git;
pub use send_question::ask_beebo;
pub use take_notes::write_beebo_notes;
