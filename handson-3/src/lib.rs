pub fn holiday_planning(cities: Vec<Vec<u32>>, days_no: usize) -> u32 {
    let n = cities.len();

    let mut dp = vec![vec![0; days_no + 1]; n + 1]; // dp table

    for i in 1..(n + 1) {
        for j in 1..(days_no + 1) {
            let mut total_attractions = 0;

            for k in 0..days_no {
                if j as i32 - k as i32 > 0 {
                    total_attractions += cities[i - 1][k];
                    dp[i][j] = std::cmp::max(dp[i][j], total_attractions + dp[i - 1][j - k - 1]);
                }
            }

            dp[i][j] = std::cmp::max(dp[i][j], dp[i - 1][j]);
        }
    }

    dp[n][days_no]
}

pub fn design_a_course(mut topics: Vec<(i32, i32)>) -> usize {
    let n = topics.len();

    topics.sort_by(|a, b| match a.0.cmp(&b.0) {
        std::cmp::Ordering::Equal => b.1.cmp(&a.1),
        ord => ord,
    });

    let mut lis = vec![];
    lis.push(topics[0]);

    for topic in topics.iter().take(n).skip(1) {
        if topic.1 > lis[lis.len() - 1].1 {
            lis.push(*topic);
        } else {
            let mut l = 0;
            let mut r = lis.len() - 1;

            while l < r {
                let mid = l + (r - l) / 2;

                if lis[mid].1 < topic.1 {
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }

            lis[l] = *topic;
        }
    }

    lis.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_holiday_planning() {
        let cities = vec![vec![3, 2, 1], vec![3, 1, 1]];
        let days_no = 3;

        assert_eq!(holiday_planning(cities, days_no), 8);
    }

    #[test]
    fn test_design_a_course() {
        let topics = vec![(0, 3), (99, 1), (11, 20), (1, 2), (10, 5)];

        assert_eq!(design_a_course(topics), 3);
    }
}
