//! What the message pill says while a panel laid out in numbered steps waits
//! on one of them: the tool, the step's number, and what it asks for.

use cao_prefs::Command;

use crate::lang::Catalogue;
use crate::wording::command;

pub fn asks(lang: &Catalogue, tool: Command, number: usize, asked: &str) -> String {
    let tool = command::label(lang, tool);
    lang.t_with(
        "steps.asks",
        &[
            ("tool", &tool),
            ("number", &number.to_string()),
            ("asked", asked),
        ],
    )
}
