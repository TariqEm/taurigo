//! One module per command domain. Commands stay thin per `CLAUDE.md`: parse input,
//! call into `services/`, map the result to a response — no business logic here.

pub mod settings;
