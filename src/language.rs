#[derive(Debug, PartialEq, Eq)]
pub enum Lang {
    En,
    Jp,
}

impl Lang {
    pub fn from_source(source: &str) -> Lang {
        let alphabetic_len = source.chars().filter(|c| c.is_ascii_alphabetic()).count();
        let other_len = source.chars().filter(|c| !c.is_ascii_alphabetic()).count();

        if alphabetic_len > other_len {
            Lang::En
        } else {
            Lang::Jp
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_en() {
        assert_eq!(Lang::En, Lang::from_source("derive implementations of the"));
        assert_eq!(Lang::En, Lang::from_source("derive あいう"));
    }

    #[test]
    fn detect_jp() {
        assert_eq!(Lang::Jp, Lang::from_source("明日ちゃんのセーラー服"));
        assert_eq!(Lang::Jp, Lang::from_source("プリンセスコネクト Re dive"));
    }
}
