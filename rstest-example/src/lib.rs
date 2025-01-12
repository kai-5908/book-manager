use mockall::predicate::*;
use mockall::*;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use rstest::{fixture, rstest};

    #[fixture]
    fn sample_fixture() -> i32 {
        24
    }

    #[rstest]
    fn sample_test(sample_fixture: i32) {
        assert_eq!(sample_fixture * 2, 48)
    }
}

pub fn sub(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod sub_tests {
    use super::*;
    use rstest::rstest;
    #[rstest]
    #[case(10, 0, 10)]
    #[case(95, 5, 100)]
    fn test_sub(#[case] a: i32, #[case] b: i32, #[case] expected: i32) {
        assert_eq!(sub(a, b), expected)
    }
}

#[automock]
trait SampleTrait {
    fn sample_method(&self, x: u32) -> u32;
}

fn sample_func(x: &dyn SampleTrait, v: u32) -> u32 {
    x.sample_method(v)
}
fn main() {
    let mut mock = MockSampleTrait::new();
    mock.expect_sample_method().returning(|x| x + 1);
    assert_eq!(10, sample_func(&mock, 9))
}
