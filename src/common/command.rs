use serde::{Deserialize, Serialize};

/// a struct which supports serialization and deserialization
#[derive(Serialize, Deserialize, Debug)]
pub enum Command {
    /// for set command
    /// Represents a string value.
    ///
    /// This type is used in the `SET` variant of the `Command` enum.
    SET(String, String),
    /// for rm command
    RM(String),
}