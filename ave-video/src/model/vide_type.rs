#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VideoContainerType {
    Mp4,
    Flv,
    M3u8,
    F4v,
    Webm,
    M4v,
    Mov,
    Rm,
    Rmvb,
    Wmv,
    Avi,
    Asf,
    Mpg,
    Mpeg,
    Mpe,
    Ts,
    Div,
    Dv,
    Vob,
    Dat,
    Mkv,
    Swf,
    Cpk,
    Dirac,
    Ram,
    Qt,
    Fli,
    Flc,
    Mod,
}

impl VideoContainerType {
    pub(crate) const ALL: [VideoContainerType; 29] = [
        VideoContainerType::Mp4,
        VideoContainerType::Flv,
        VideoContainerType::F4v,
        VideoContainerType::M3u8,
        VideoContainerType::Webm,
        VideoContainerType::M4v,
        VideoContainerType::Mov,
        VideoContainerType::Rm,
        VideoContainerType::Rmvb,
        VideoContainerType::Wmv,
        VideoContainerType::Avi,
        VideoContainerType::Asf,
        VideoContainerType::Mpg,
        VideoContainerType::Mpeg,
        VideoContainerType::Mpe,
        VideoContainerType::Ts,
        VideoContainerType::Div,
        VideoContainerType::Dv,
        VideoContainerType::Vob,
        VideoContainerType::Dat,
        VideoContainerType::Mkv,
        VideoContainerType::Swf,
        VideoContainerType::Cpk,
        VideoContainerType::Dirac,
        VideoContainerType::Ram,
        VideoContainerType::Qt,
        VideoContainerType::Fli,
        VideoContainerType::Flc,
        VideoContainerType::Mod
    ];

    //获取所有类型
    pub(crate) fn get_all_type(&self) -> String {
        let all_data = Self::ALL;
        let mut result = String::new();
        for all_datum in all_data.iter() {
            result.push_str(all_datum.to_string().as_str());
            result.push_str(",");
        }
        result
    }
}

impl Default for VideoContainerType {
    fn default() -> VideoContainerType {
        VideoContainerType::Mp4
    }
}

impl std::fmt::Display for VideoContainerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VideoContainerType::Mp4 => "mp4",
                VideoContainerType::Flv => "flv",
                VideoContainerType::M3u8 => "m3u8",
                VideoContainerType::F4v => "f4v",
                VideoContainerType::Webm => "webm",
                VideoContainerType::M4v => "m4v",
                VideoContainerType::Mov => "mov",
                VideoContainerType::Rm => "rm",
                VideoContainerType::Rmvb => "rmvb",
                VideoContainerType::Wmv => "wmv",
                VideoContainerType::Avi => "avi",
                VideoContainerType::Asf => "asf",
                VideoContainerType::Mpg => "mpg",
                VideoContainerType::Mpeg => "mpeg",
                VideoContainerType::Mpe => "mpe",
                VideoContainerType::Ts => "ts",
                VideoContainerType::Div => "div",
                VideoContainerType::Dv => "dv",
                VideoContainerType::Vob => "vob",
                VideoContainerType::Dat => "dat",
                VideoContainerType::Mkv => "mkv",
                VideoContainerType::Swf => "swf",
                VideoContainerType::Cpk => "cpk",
                VideoContainerType::Dirac => "dirac",
                VideoContainerType::Ram => "ram",
                VideoContainerType::Qt => "qt",
                VideoContainerType::Fli => "fli",
                VideoContainerType::Flc => "flc",
                VideoContainerType::Mod => "mod"
            }
        )
    }
}