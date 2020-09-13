use proptest::prelude::*;

use super::*;

fn runtime_value_strategy() -> impl Strategy<Value = RuntimeValue> {
    prop_oneof![
        any::<u64>().prop_map(|n| RuntimeValue::Number(Number::Natural(n))),
        any::<f64>().prop_map(|f| RuntimeValue::Number(Number::Float(f))),
        any::<i64>().prop_map(|i| RuntimeValue::Number(Number::Integer(i))),
        any::<bool>().prop_map(|b| RuntimeValue::Boolean(b)),
        ".*".prop_map(|s| RuntimeValue::String(s)),
    ]
}

proptest! {
    #[test]
    fn test_one_arg_format(v in runtime_value_strategy(),
                           f in r#"[^{]*\{\}[^}]*"#) {
        let stack = &mut vec![v.clone(), RuntimeValue::String(f)];
        apply_format(stack).unwrap();

        prop_assert!(stack.len() == 1);
        match &stack[0] {
            RuntimeValue::String(s) => prop_assert!(s.contains(&v.to_string())),
            _ => prop_assert!(false),
        }
    }

    #[test]
    fn test_two_arg_format(v1 in runtime_value_strategy(),
                           v2 in runtime_value_strategy(),
                           f in r#"[^{]*\{\}[^}]*\{\}[^}]*"#) {
        let stack = &mut vec![v1.clone(), v2.clone(), RuntimeValue::String(f)];
        apply_format(stack).unwrap();

        prop_assert_eq!(stack.len(), 1);
        match &stack[0] {
            RuntimeValue::String(s) => {
                prop_assert!(s.contains(&v1.to_string()));
                prop_assert!(s.contains(&v2.to_string()));
            }
            _ => prop_assert!(false),
        }
    }
}
