use super::*;

impl<T: AsRef<str> + From<String>> CheckKeyword<T> for T {
    fn is_keyword(&self) -> bool {
        KEYWORDS.contains(&self.as_ref())
    }

    fn is_raw_restricted(&self) -> bool {
        RAW_RESTRICTED.contains(&self.as_ref())
    }

    fn into_safe(self) -> Self {
        if self.is_keyword() {
            if self.is_raw_restricted() {
                let safe = format!("{}_", self.as_ref());
                safe.into()
            } else {
                let safe = format!("r#{}", self.as_ref());
                safe.into()
                }
        } else {
            self
        }
    }

}

impl CheckKeyword<String> for &str {
    fn is_keyword(&self) -> bool {
        KEYWORDS.contains(self)
    }

    fn is_raw_restricted(&self) -> bool {
        RAW_RESTRICTED.contains(self)
    }

    fn into_safe(self) -> String {
        if self.is_keyword() {
            if self.is_raw_restricted() {
                let safe = format!("{}_", self);
                safe.into()
            } else {
                let safe = format!("r#{}", self);
                safe.into()
            }
        } else {
            self.into()
        } 
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_keyword() {
        assert!(String::from("match").is_keyword());
        assert!(!"hello".is_keyword());

        assert_eq!(String::from("async").is_keyword(), cfg!(feature = "2018"));
    }

    #[test]
    fn is_raw_restricted() {
        assert!(String::from("self").is_raw_restricted());
        assert!(!"match".is_raw_restricted());

        assert_eq!(String::from("async").is_keyword(), cfg!(feature = "2018"));
    }

    #[test]
    fn into_safe() {
        assert_eq!(String::from("match").into_safe(), "r#match");
        assert_eq!(String::from("crate").into_safe(), "crate_");
        assert_eq!("asdf".into_safe(), "asdf");

        assert_eq!("await".into_safe(),
            if cfg!(feature = "2018") {
                "r#await"
            } else {
                "await"
            }
        )
    }
}