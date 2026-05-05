use tensor_dev_formatter_meter::{classify, score, Signal};
#[test]
fn fixture_decisions() {
    let signal = Signal { demand: 61, capacity: 105, latency: 22, risk: 23, weight: 10 };
    assert_eq!(score(signal), 8);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 97, capacity: 99, latency: 15, risk: 25, weight: 10 };
    assert_eq!(score(signal), 88);
    assert_eq!(classify(signal), "review");
    let signal = Signal { demand: 94, capacity: 99, latency: 13, risk: 17, weight: 8 };
    assert_eq!(score(signal), 140);
    assert_eq!(classify(signal), "review");
}
