use std::fmt;

/// list of results
#[derive(Serialize, Deserialize, Debug)]
pub struct Coala {
    results: CoalaCli
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CoalaCli {
    cli: Vec<CoalaResults>
}

/// position of affected code
#[derive(Serialize, Deserialize, Debug)]
pub struct Position {
    /// the name of the file
    file: String,
    /// line number
    line: u32,
    /// column number
    column: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AffectedCode {
    /// the name of the file
    file: String,
    /// start position of affected code
    start: Position,
    /// end position of affected code
    end: Position
}

enum_number!(Severity {
    Info = 0,
    Normal = 1,
    Major = 2,
});


#[derive(Serialize, Deserialize, Debug)]
pub struct CoalaResults {
    /// usually the name of the bear
    origin: String,
    /// message to be displayed to the user
    message: String,
    /// contains SourceRange objects
    affected_code: Vec<AffectedCode>,
    /// severity of the result
    severity: Severity,
    /// message to be shown in DEBUG log
    debug_msg: String,
    /// additional info to be displayed
    additional_info: String
}
