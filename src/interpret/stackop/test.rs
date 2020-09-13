use proptest::prelude::*;

use super::*;

proptest! {
    #[test]
    fn test_index_pick(index in 0u64..1000u64) {
        let stack = &mut vec![
            RuntimeValue::Number(Number::Natural(100)),
            RuntimeValue::Number(Number::Natural(200)),
            RuntimeValue::Number(Number::Natural(300)),
            RuntimeValue::Number(Number::Natural(index)),
        ];

        // NOTE: stack[index] should never panic!
        match apply_pick(stack) {
            Ok(()) => prop_assert_eq!(stack.len(), 4),
            Err(msg) => {
                prop_assert_eq!(stack.len(), 3);
                prop_assert_eq!(
                    format!("Invalid index {} for pick into stack of size 3",
                            index),
                    msg);
            }
        }
    }
}
