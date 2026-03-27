use presto_core::{FunctionRegistry, PrestoResult};

pub mod window {
    use presto_core::{PrestoError, PrestoResult};

    pub fn row_number(len: usize) -> Vec<i64> {
        (1..=len as i64).collect()
    }

    pub fn rank<T: PartialEq>(ordered_values: &[T]) -> Vec<i64> {
        if ordered_values.is_empty() {
            return Vec::new();
        }
        let mut out = Vec::with_capacity(ordered_values.len());
        let mut current_rank = 1i64;
        out.push(current_rank);
        for i in 1..ordered_values.len() {
            if ordered_values[i] != ordered_values[i - 1] {
                current_rank = (i + 1) as i64;
            }
            out.push(current_rank);
        }
        out
    }

    pub fn dense_rank<T: PartialEq>(ordered_values: &[T]) -> Vec<i64> {
        if ordered_values.is_empty() {
            return Vec::new();
        }
        let mut out = Vec::with_capacity(ordered_values.len());
        let mut current_rank = 1i64;
        out.push(current_rank);
        for i in 1..ordered_values.len() {
            if ordered_values[i] != ordered_values[i - 1] {
                current_rank += 1;
            }
            out.push(current_rank);
        }
        out
    }

    pub fn percent_rank<T: PartialEq>(ordered_values: &[T]) -> Vec<f64> {
        let n = ordered_values.len();
        if n == 0 {
            return Vec::new();
        }
        if n == 1 {
            return vec![0.0];
        }
        rank(ordered_values)
            .into_iter()
            .map(|r| (r - 1) as f64 / (n as f64 - 1.0))
            .collect()
    }

    pub fn cume_dist<T: PartialEq>(ordered_values: &[T]) -> Vec<f64> {
        let n = ordered_values.len();
        if n == 0 {
            return Vec::new();
        }

        let mut out = vec![0.0; n];
        let mut i = 0usize;
        while i < n {
            let mut j = i + 1;
            while j < n && ordered_values[j] == ordered_values[i] {
                j += 1;
            }
            let value = j as f64 / n as f64;
            for slot in out.iter_mut().take(j).skip(i) {
                *slot = value;
            }
            i = j;
        }
        out
    }

    pub fn ntile(len: usize, buckets: i64) -> PrestoResult<Vec<i64>> {
        if buckets <= 0 {
            return Err(PrestoError::InvalidArgument("ntile buckets must be > 0"));
        }
        let buckets = buckets as usize;
        let mut out = Vec::with_capacity(len);

        for i in 0..len {
            let bucket = ((i * buckets) / len) + 1;
            out.push(bucket as i64);
        }
        Ok(out)
    }

    pub fn lag<T: Clone>(
        values: &[T],
        offset: Option<usize>,
        default: Option<T>,
    ) -> Vec<Option<T>> {
        let offset = offset.unwrap_or(1);
        values
            .iter()
            .enumerate()
            .map(|(i, _)| {
                if i >= offset {
                    Some(values[i - offset].clone())
                } else {
                    default.clone()
                }
            })
            .collect()
    }

    pub fn lead<T: Clone>(
        values: &[T],
        offset: Option<usize>,
        default: Option<T>,
    ) -> Vec<Option<T>> {
        let offset = offset.unwrap_or(1);
        values
            .iter()
            .enumerate()
            .map(|(i, _)| {
                if i + offset < values.len() {
                    Some(values[i + offset].clone())
                } else {
                    default.clone()
                }
            })
            .collect()
    }

    pub fn first_value<T: Clone>(values: &[T]) -> Option<T> {
        values.first().cloned()
    }

    pub fn last_value<T: Clone>(values: &[T]) -> Option<T> {
        values.last().cloned()
    }

    pub fn nth_value<T: Clone>(values: &[T], offset: usize) -> Option<T> {
        if offset == 0 {
            return None;
        }
        values.get(offset - 1).cloned()
    }
}

pub fn register(registry: &mut FunctionRegistry) -> PrestoResult<()> {
    let window_functions = [
        "cume_dist",
        "dense_rank",
        "first_value",
        "lag",
        "last_value",
        "lead",
        "nth_value",
        "ntile",
        "percent_rank",
        "rank",
        "row_number",
    ];

    for name in window_functions {
        registry.register_window(name);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::window;

    #[test]
    fn ranking_functions_work() {
        let values = vec![1, 1, 2, 4, 4];
        assert_eq!(window::row_number(values.len()), vec![1, 2, 3, 4, 5]);
        assert_eq!(window::rank(&values), vec![1, 1, 3, 4, 4]);
        assert_eq!(window::dense_rank(&values), vec![1, 1, 2, 3, 3]);
    }

    #[test]
    fn offset_and_percentile_windows_work() {
        let values = vec![10, 20, 30, 40];
        assert_eq!(
            window::lag(&values, Some(1), None),
            vec![None, Some(10), Some(20), Some(30)]
        );
        assert_eq!(
            window::lead(&values, Some(1), None),
            vec![Some(20), Some(30), Some(40), None]
        );
        assert_eq!(
            window::percent_rank(&values),
            vec![0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0]
        );
    }
}
