pub enum TimeGet {
    Today,
    Week,
    Yesterday,
}

pub enum TaskGet {
    Last,
    Sprint,
}

pub struct TimeTrack<'a> {
    pub mode: TimeTrackFirstArg<'a>,
    pub duration: u32,
}

pub enum TimeTrackFirstArg<'a> {
    Last,
    TaskId(&'a str),
}

