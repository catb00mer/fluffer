use url::Url;

/// 📝 Represent a type as gemtext.
pub trait AsGemtext {
    fn as_gemtext(&self) -> String;
}

impl AsGemtext for Vec<u8> {
    /// Implementation to parse gemini bytes for gemtext.
    ///
    /// If the bytes begin with anything other than `20
    /// text/gemini...`, then the status is returned inside
    /// a code block.
    fn as_gemtext(&self) -> String {
        if let Ok(b) = std::str::from_utf8(&self) {
            if let Some((header, content)) = b.split_once("\r\n") {
                if let Some(pos) = header.find("20 text/gemini") {
                    if pos == 0 {
                        return content.to_string();
                    }
                }
                return format!("```\n{header}\n```");
            }
            return String::from("```\ninvalid gemini response\n```");
        }
        String::from("```\ninvalid utf8\n```")
    }
}

impl AsGemtext for Url {
    fn as_gemtext(&self) -> String {
        format!("=> {} {}", self, self)
    }
}
