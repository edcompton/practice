use advent_of_code::*;
use advent_of_code::error::Error;

#[test]
fn day_two_expected_result() -> Result<(),Error> {
    let result = day_two::run()?;
    assert_eq!(result, vec![3760627, 19690720]);
    Ok(())
}
