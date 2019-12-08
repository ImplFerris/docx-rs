use crate::documents::BuildXML;
use crate::xml_builder::*;

#[derive(Debug, Clone)]
pub struct LevelText {
    val: String,
}

impl LevelText {
    pub fn new(val: impl Into<String>) -> Self {
        Self { val: val.into() }
    }
}

impl BuildXML for LevelText {
    fn build(&self) -> Vec<u8> {
        let b = XMLBuilder::new();
        b.level_text(&self.val).build()
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[cfg(test)]
    use pretty_assertions::assert_eq;
    use std::str;

    #[test]
    fn test_level_text() {
        let c = LevelText::new("%4.");
        let b = c.build();
        assert_eq!(str::from_utf8(&b).unwrap(), r#"<w:lvlText w:val="%4." />"#);
    }
}
