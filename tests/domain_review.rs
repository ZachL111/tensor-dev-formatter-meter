use tensor_dev_formatter_meter::domain_review::{review_lane, review_score, DomainCase};

#[test]
fn domain_review_case_is_stable() {
    let case = DomainCase { signal: 73, slack: 24, drag: 31, confidence: 64 };
    assert_eq!(review_score(case), 141);
    assert_eq!(review_lane(case), "ship");
}
