#[derive(PartialEq, Debug)]
pub enum Since {
    Daily,
    Weekly,
    Monthly,
}

impl Since {
    pub fn to_string(&self) -> &str {
        match self {
            Self::Daily => "daily",
            Self::Weekly => "weekly",
            Self::Monthly => "monthly",
        }
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "daily" => Self::Daily,
            "weekly" => Self::Weekly,
            "monthly" => Self::Monthly,
            _ => Self::Daily,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::gtrend::Since;

    #[test]
    fn test_since_to_string() {
        let x: &str = Since::Daily.to_string();
        assert_eq!(x, "daily");
    }

    #[test]
    fn test_since_from_string() {
        let x: Since = Since::from_string("daily");
        assert_eq!(x, Since::Daily);
    }
}
