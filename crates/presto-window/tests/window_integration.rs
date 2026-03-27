use presto_window::window;

#[test]
fn ranking_value_scenarios_match_expected_sequences() {
    let ordered = vec![10, 10, 20, 30, 30, 30, 40];
    assert_eq!(window::rank(&ordered), vec![1, 1, 3, 4, 4, 4, 7]);
    assert_eq!(window::dense_rank(&ordered), vec![1, 1, 2, 3, 3, 3, 4]);
    assert_eq!(
        window::cume_dist(&ordered),
        vec![
            2.0 / 7.0,
            2.0 / 7.0,
            3.0 / 7.0,
            6.0 / 7.0,
            6.0 / 7.0,
            6.0 / 7.0,
            1.0
        ]
    );
}

#[test]
fn lag_lead_ntile_scenarios_are_stable() {
    let values = vec!["a", "b", "c", "d", "e"];
    assert_eq!(
        window::lag(&values, Some(2), Some("x")),
        vec![Some("x"), Some("x"), Some("a"), Some("b"), Some("c")]
    );
    assert_eq!(
        window::lead(&values, Some(2), Some("x")),
        vec![Some("c"), Some("d"), Some("e"), Some("x"), Some("x")]
    );
    assert_eq!(window::ntile(values.len(), 3).unwrap(), vec![1, 1, 2, 2, 3]);
}
