// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

//M NOT DONE

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Explanation: Check if the value is negative and return the appropriate error.
        if value < 0 {
            return Err(CreationError::Negative);
        }
        // Explanation: Check if the value is zero and return the appropriate error.
        if value == 0 {
            return Err(CreationError::Zero);
        }
        // Explanation: If the value is positive and non-zero, return the wrapped value in an Ok.
        Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}

