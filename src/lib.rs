use chrono::{FixedOffset, DateTime};

pub fn apply_policy<'a, T>(now: DateTime<FixedOffset>, policy: &[&DateTime<FixedOffset>], list: &'a[(DateTime<FixedOffset>, T)]) -> Vec<(&'a DateTime<FixedOffset>, &'a T)> {
    todo!()
}

#[cfg(test)]
mod tests {
    use chrono::Utc;

    use super::*;

    #[test]
    fn empty_policy() {
        let list: Vec<(DateTime<FixedOffset>, String)> = vec![(Utc::now().into(), "test".to_string())];
        let result = apply_policy(Utc::now().into(), &[], &list[..]);
        assert_eq!(result.len(), 1);
    }
}
