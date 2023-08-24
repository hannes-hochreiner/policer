use chrono::{DateTime, Duration, Utc};

/// Return a list of dates to be deleted
///
/// The policy vector is taken to create buckets, where the entries in the vector are the limits.
/// Two additional buckets are created for the time before the last entry of the vector and after the first entry.
/// All backups are assigned to these buckets starting with the nearest past.
/// If there is no entry found for a bucket, the next newest entry will be used.
/// If there are multiple entries in one bucket, the oldest entry will be kept.
/// For the bucket with the oldest entries, only the newest entry will be kept.
///
/// * `now` - current date and time
/// * `policy` - a vector of durations
/// * `list` - a vector of tuples of dates and objects
///
pub fn police<'a, T>(
    now: &DateTime<Utc>,
    policy: &[Duration],
    list: &'a [(DateTime<Utc>, T)],
) -> Vec<&'a (DateTime<Utc>, T)> {
    let mut bucket_vec: Vec<&(DateTime<Utc>, T)> = Vec::new();
    let mut policy: Vec<&Duration> = policy.iter().collect();

    policy.sort();

    let mut policy_iter = policy.iter();
    let mut policy_elem = policy_iter.next();
    let mut res: Vec<&(DateTime<Utc>, T)> = Vec::new();
    let mut list: Vec<&(DateTime<Utc>, T)> = list.iter().collect();

    list.sort_by_key(|&e| e.0);

    for &item in list.iter().rev() {
        match policy_elem {
            Some(&p) => {
                if *now - item.0 > *p {
                    if !bucket_vec.is_empty() {
                        bucket_vec.pop();
                        res.append(&mut bucket_vec);
                        bucket_vec.push(item);
                    }

                    policy_elem = policy_iter.next();
                } else {
                    bucket_vec.push(item);
                }
            }
            None => {
                bucket_vec.push(item);
            }
        }
    }

    if !bucket_vec.is_empty() {
        bucket_vec.remove(0);
        res.append(&mut bucket_vec);
    }

    while !res.is_empty() && list.len() - res.len() <= policy.len() {
        res.pop();
    }

    res
}

#[cfg(test)]
mod tests {
    use chrono::{TimeZone, Utc};

    use super::*;

    #[test]
    fn empty_policy_one_element() {
        let list: Vec<(DateTime<Utc>, String)> = vec![(Utc::now().into(), "test".to_string())];
        let result = police(&Utc::now().into(), &[], &list[..]);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn empty_policy_two_elements() {
        let list: Vec<(DateTime<Utc>, String)> = vec![
            (Utc::now().into(), "test1".to_string()),
            (Utc::now().into(), "test2".to_string()),
        ];
        let result = police(&Utc::now().into(), &[], &list[..]);
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, "test1");
    }

    #[test]
    fn one_policy_two_elements() {
        let now = Utc.with_ymd_and_hms(2022, 10, 29, 0, 0, 0).unwrap();
        let policy: Vec<Duration> = vec![Duration::days(1)];
        let list: Vec<(DateTime<Utc>, String)> = vec![
            (
                Utc.with_ymd_and_hms(2022, 10, 22, 0, 0, 0).unwrap().into(),
                "test1".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 21, 0, 0, 0).unwrap().into(),
                "test2".to_string(),
            ),
        ];
        let result = police(&now.into(), &policy, &list[..]);

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn one_policy_three_elements() {
        let now = Utc.with_ymd_and_hms(2022, 10, 29, 0, 0, 0).unwrap();
        let policy: Vec<Duration> = vec![Duration::days(1)];
        let list: Vec<(DateTime<Utc>, String)> = vec![
            (
                Utc.with_ymd_and_hms(2022, 10, 22, 0, 0, 0).unwrap().into(),
                "test1".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 21, 0, 0, 0).unwrap().into(),
                "test2".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 20, 0, 0, 0).unwrap().into(),
                "test3".to_string(),
            ),
        ];
        let result = police(&now.into(), &policy, &list[..]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, "test3");
    }

    #[test]
    fn one_policy_three_elements_younger() {
        let now = Utc.with_ymd_and_hms(2022, 10, 29, 0, 0, 0).unwrap();
        let policy: Vec<Duration> = vec![Duration::days(100)];
        let list: Vec<(DateTime<Utc>, String)> = vec![
            (
                Utc.with_ymd_and_hms(2022, 10, 22, 0, 0, 0).unwrap().into(),
                "test1".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 21, 0, 0, 0).unwrap().into(),
                "test2".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 20, 0, 0, 0).unwrap().into(),
                "test3".to_string(),
            ),
        ];
        let result = police(&now.into(), &policy, &list[..]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, "test2");
    }

    #[test]
    fn one_policy_four_elements() {
        let now = Utc.with_ymd_and_hms(2022, 10, 29, 0, 0, 0).unwrap();
        let policy: Vec<Duration> = vec![Duration::days(1)];
        let list: Vec<(DateTime<Utc>, String)> = vec![
            (
                Utc.with_ymd_and_hms(2022, 10, 22, 0, 0, 0).unwrap().into(),
                "test1".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 21, 0, 0, 0).unwrap().into(),
                "test2".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 11, 20, 0, 0, 0).unwrap().into(),
                "test3".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 12, 20, 0, 0, 0).unwrap().into(),
                "test4".to_string(),
            ),
        ];
        let result = police(&now.into(), &policy, &list[..]);

        assert_eq!(result.len(), 2);
        assert_eq!(result[0].1, "test4");
        assert_eq!(result[1].1, "test2");
    }

    #[test]
    fn two_policies_four_elements_1() {
        let now = Utc.with_ymd_and_hms(2022, 10, 29, 0, 0, 0).unwrap();
        let policy: Vec<Duration> = vec![Duration::days(1), Duration::days(7)];
        let list: Vec<(DateTime<Utc>, String)> = vec![
            (
                Utc.with_ymd_and_hms(2022, 10, 28, 12, 0, 0).unwrap().into(),
                "test3".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 26, 0, 0, 0).unwrap().into(),
                "test4".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 22, 0, 0, 0).unwrap().into(),
                "test1".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 21, 0, 0, 0).unwrap().into(),
                "test2".to_string(),
            ),
        ];
        let result = police(&now.into(), &policy, &list[..]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, "test4");
    }

    #[test]
    fn two_policies_four_elements_2() {
        let now = Utc.with_ymd_and_hms(2022, 10, 29, 0, 0, 0).unwrap();
        let policy: Vec<Duration> = vec![Duration::days(1), Duration::days(7)];
        let list: Vec<(DateTime<Utc>, String)> = vec![
            (
                Utc.with_ymd_and_hms(2022, 10, 20, 0, 0, 0).unwrap().into(),
                "test1".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 19, 0, 0, 0).unwrap().into(),
                "test2".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 28, 12, 0, 0).unwrap().into(),
                "test3".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 26, 0, 0, 0).unwrap().into(),
                "test4".to_string(),
            ),
        ];
        let result = police(&now.into(), &policy, &list[..]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, "test2");
    }

    #[test]
    fn two_policies_four_elements_3() {
        let now = Utc.with_ymd_and_hms(2022, 10, 29, 0, 0, 0).unwrap();
        let policy: Vec<Duration> = vec![Duration::days(1), Duration::days(7)];
        let list: Vec<(DateTime<Utc>, String)> = vec![
            (
                Utc.with_ymd_and_hms(2022, 10, 28, 13, 0, 0).unwrap().into(),
                "test4".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 28, 12, 0, 0).unwrap().into(),
                "test3".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 20, 0, 0, 0).unwrap().into(),
                "test1".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 19, 0, 0, 0).unwrap().into(),
                "test2".to_string(),
            ),
        ];
        let result = police(&now.into(), &policy, &list[..]);

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].1, "test4");
    }

    #[test]
    fn two_policies_three_elements() {
        let now = Utc.with_ymd_and_hms(2022, 10, 29, 0, 0, 0).unwrap();
        let policy: Vec<Duration> = vec![Duration::days(1), Duration::days(7)];
        let list: Vec<(DateTime<Utc>, String)> = vec![
            (
                Utc.with_ymd_and_hms(2022, 10, 28, 13, 0, 0).unwrap().into(),
                "test4".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 28, 12, 0, 0).unwrap().into(),
                "test3".to_string(),
            ),
            (
                Utc.with_ymd_and_hms(2022, 10, 20, 0, 0, 0).unwrap().into(),
                "test1".to_string(),
            ),
        ];
        let result = police(&now.into(), &policy, &list[..]);

        assert_eq!(result.len(), 0);
    }
}
