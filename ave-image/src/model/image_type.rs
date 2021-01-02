#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageType {
    Png,
    Jpg,
    Jpeg,
    Gif,
    WebP,
    Pnm,
    Tiff,
    Tga,
    Dds,
    Bmp,
    Ico,
    Hdr,
    Farbfeld,
    Avif,
}

impl ImageType {
    pub(crate) const ALL: [ImageType; 14] = [
        ImageType::Png,
        ImageType::Jpg,
        ImageType::Jpeg,
        ImageType::Gif,
        ImageType::WebP,
        ImageType::Pnm,
        ImageType::Tiff,
        ImageType::Tga,
        ImageType::Dds,
        ImageType::Bmp,
        ImageType::Ico,
        ImageType::Hdr,
        ImageType::Farbfeld,
        ImageType::Avif,
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

impl Default for ImageType {
    fn default() -> ImageType {
        ImageType::Jpg
    }
}

impl std::fmt::Display for ImageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ImageType::Jpg => "jpg",
                ImageType::Png => "png",
                ImageType::Jpeg => "jpeg",
                ImageType::Gif => "gif",
                ImageType::WebP => "webp",
                ImageType::Pnm => "pnm",
                ImageType::Tiff => "tiff",
                ImageType::Tga => "tga",
                ImageType::Dds => "dds",
                ImageType::Bmp => "bmp",
                ImageType::Ico => "ico",
                ImageType::Hdr => "hdr",
                ImageType::Farbfeld => "farbfeld",
                ImageType::Avif => "avif",
            }
        )
    }
}