pub const CELL_SIZE_X: usize = 2;
pub const CELL_SIZE_Y: usize = 4;
pub const CELL_SIZE: usize = CELL_SIZE_X * CELL_SIZE_Y;

pub const BRAILLE_BASE: u32 = 10240;

pub const VIDEO_FORMATS: &'static [&str] = &[
    "webm", "mkv", "flv", "vob", "ogv", "ogg", "rrc", "gifv", "mng", "mov", "avi", "qt", "wmv",
    "yuv", "rm", "asf", "amv", "mp4", "m4p", "m4v", "mpg", "mp2", "mpeg", "mpe", "mpv", "m4v",
    "svi", "3gp", "3g2", "mxf", "roq", "nsv", "flv", "f4v", "f4p", "f4a", "f4b", "mod",
];
