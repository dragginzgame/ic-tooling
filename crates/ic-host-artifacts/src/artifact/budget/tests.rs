use super::*;
use std::io;
#[test]
fn aggregate_reads_charge_success_growth_and_partial_failures() {
    let mut budget = ReadBudget::new(5);
    assert_eq!(budget.read(&b"abc"[..], 3, 5).unwrap(), b"abc");
    assert_eq!(budget.remaining(), 2);
    assert!(matches!(
        budget.read(&b"xyz"[..], 3, 5),
        Err(BudgetReadError::AggregateLimit { maximum: 5 })
    ));
    assert_eq!(budget.remaining(), 2);
    assert!(matches!(
        budget.read(&b"xyz"[..], 0, 5),
        Err(BudgetReadError::AggregateLimit { maximum: 5 })
    ));
    assert_eq!(budget.remaining(), 0);
    let mut budget = ReadBudget::new(10);
    assert!(matches!(
        budget.read(&b"abcd"[..], 4, 3),
        Err(BudgetReadError::FileLimit {
            actual: 4,
            maximum: 3
        })
    ));
    assert_eq!(budget.remaining(), 10);
    assert!(matches!(
        budget.read(&b"abcd"[..], 0, 3),
        Err(BudgetReadError::FileLimit {
            actual: 4,
            maximum: 3
        })
    ));
    assert_eq!(budget.remaining(), 6);
    assert!(matches!(
        budget.read((&b"ab"[..]).chain(Fails), 0, 2),
        Err(BudgetReadError::Read(_))
    ));
    assert_eq!(budget.remaining(), 3);
}
struct Fails;
impl io::Read for Fails {
    fn read(&mut self, _: &mut [u8]) -> io::Result<usize> {
        Err(io::Error::other("injected read failure"))
    }
}
#[test]
fn zero_and_unbounded_allowances_admit_empty_and_small_streams() {
    assert_eq!(
        ReadBudget::new(0).read(io::empty(), 0, 0).unwrap(),
        Vec::<u8>::new()
    );
    assert_eq!(
        ReadBudget::new(u64::MAX)
            .read(&b"x"[..], 1, u64::MAX)
            .unwrap(),
        b"x"
    );
}
