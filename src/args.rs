pub enum TimeGet {
    Today,
    Week,
    Yesterday,
}

pub enum TaskGet {
    Last,
    Sprint,
}

pub enum TimeTrack<'a> {
    Last,
    TaskId(&'a str),
}
