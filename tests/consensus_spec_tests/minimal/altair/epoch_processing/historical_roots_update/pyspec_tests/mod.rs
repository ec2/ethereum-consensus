// WARNING!
// This file was generated with `gen-tests`. Do NOT edit manually.

use crate::spec_test_runners::epoch_processing::HistoricalRootsUpdateTestCase;
use crate::test_utils::TestCase;

#[test]
fn test_historical_root_accumulator() {
    let  test_case = HistoricalRootsUpdateTestCase::from("consensus-spec-tests/tests/minimal/altair/epoch_processing/historical_roots_update/pyspec_tests/historical_root_accumulator");
    test_case.execute();
}
