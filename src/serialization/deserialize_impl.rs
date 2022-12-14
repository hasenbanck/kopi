use crate::{
    error::{create_type_error, TypeError},
    traits::Deserialize,
    value::{BigInt, Boolean, Integer, Number, Value, ValueScope},
};

impl<'scope> Deserialize<'scope> for () {
    #[inline(always)]
    fn deserialize(
        _scope: &mut ValueScope<'scope>,
        _value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        Ok(())
    }
}

impl<'scope> Deserialize<'scope> for bool {
    #[inline(always)]
    fn deserialize(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        if let Ok(val) = Boolean::try_from(value) {
            Ok(val.value())
        } else {
            Err(create_type_error(
                "Value can't be converted to an i8",
                scope,
                &value,
            ))
        }
    }
}

impl<'scope> Deserialize<'scope> for String {
    #[inline(always)]
    fn deserialize(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        Ok(value.to_string_representation(scope))
    }
}

impl<'scope> Deserialize<'scope> for i8 {
    #[inline(always)]
    fn deserialize(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        if let Ok(val) = Integer::try_from(value) {
            let val = i8::try_from(val.value())
                .map_err(|_| create_type_error("Value not in range for an i8", scope, &value))?;
            Ok(val)
        } else if let Ok(val) = BigInt::try_from(value) {
            let (val, lossless) = val.value_i64();
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an i8",
                    scope,
                    &value,
                ));
            }
            let val = i8::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i8", scope, &value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an i8",
                scope,
                &value,
            ))
        }
    }
}

impl<'scope> Deserialize<'scope> for i16 {
    #[inline(always)]
    fn deserialize(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        if let Ok(val) = Integer::try_from(value) {
            let val = i16::try_from(val.value())
                .map_err(|_| create_type_error("Value not in range for an i16", scope, &value))?;
            Ok(val)
        } else if let Ok(val) = BigInt::try_from(value) {
            let (val, lossless) = val.value_i64();
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an i16",
                    scope,
                    &value,
                ));
            }
            let val = i16::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i16", scope, &value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an i16",
                scope,
                &value,
            ))
        }
    }
}

impl<'scope> Deserialize<'scope> for i32 {
    #[inline(always)]
    fn deserialize(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        if let Ok(val) = Integer::try_from(value) {
            let val = i32::try_from(val.value())
                .map_err(|_| create_type_error("Value not in range for an i32", scope, &value))?;
            Ok(val)
        } else if let Ok(val) = BigInt::try_from(value) {
            let (val, lossless) = val.value_i64();
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an i32",
                    scope,
                    &value,
                ));
            }
            let val = i32::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i32", scope, &value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an i32",
                scope,
                &value,
            ))
        }
    }
}

impl<'scope> Deserialize<'scope> for i64 {
    #[inline(always)]
    fn deserialize(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        if let Ok(val) = Integer::try_from(value) {
            Ok(val.value())
        } else if let Ok(val) = BigInt::try_from(value) {
            let (val, lossless) = val.value_i64();
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an i64",
                    scope,
                    &value,
                ));
            }
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an i64",
                scope,
                &value,
            ))
        }
    }
}

impl<'scope> Deserialize<'scope> for u8 {
    #[inline(always)]
    fn deserialize(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        if let Ok(val) = Integer::try_from(value) {
            let val = u8::try_from(val.value())
                .map_err(|_| create_type_error("Value not in range for an u8", scope, &value))?;
            Ok(val)
        } else if let Ok(val) = BigInt::try_from(value) {
            let (val, lossless) = val.value_u64();
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an u8",
                    scope,
                    &value,
                ));
            }
            let val = u8::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u8", scope, &value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an u8",
                scope,
                &value,
            ))
        }
    }
}

impl<'scope> Deserialize<'scope> for u16 {
    #[inline(always)]
    fn deserialize(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        if let Ok(val) = Integer::try_from(value) {
            let val = u16::try_from(val.value())
                .map_err(|_| create_type_error("Value not in range for an u16", scope, &value))?;
            Ok(val)
        } else if let Ok(val) = BigInt::try_from(value) {
            let (val, lossless) = val.value_u64();
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an u16",
                    scope,
                    &value,
                ));
            }
            let val = u16::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u16", scope, &value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an u16",
                scope,
                &value,
            ))
        }
    }
}

impl<'scope> Deserialize<'scope> for u32 {
    #[inline(always)]
    fn deserialize(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        if let Ok(val) = Integer::try_from(value) {
            let val = u32::try_from(val.value())
                .map_err(|_| create_type_error("Value not in range for an u32", scope, &value))?;
            Ok(val)
        } else if let Ok(val) = BigInt::try_from(value) {
            let (val, lossless) = val.value_u64();
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an u32",
                    scope,
                    &value,
                ));
            }
            let val = u32::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u32", scope, &value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an u32",
                scope,
                &value,
            ))
        }
    }
}

impl<'scope> Deserialize<'scope> for u64 {
    #[inline(always)]
    fn deserialize(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        if let Ok(val) = Integer::try_from(value) {
            let val = u64::try_from(val.value())
                .map_err(|_| create_type_error("Value not in range for an u64", scope, &value))?;
            Ok(val)
        } else if let Ok(val) = BigInt::try_from(value) {
            let (val, lossless) = val.value_u64();
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an u64",
                    scope,
                    &value,
                ));
            }
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an u16",
                scope,
                &value,
            ))
        }
    }
}

impl<'scope> Deserialize<'scope> for f32 {
    #[inline(always)]
    fn deserialize(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        let value = Number::try_from(value)
            .map_err(|_| create_type_error("Value not a f32", scope, &value))?;
        Ok(value.value() as f32)
    }
}

impl<'scope> Deserialize<'scope> for f64 {
    #[inline(always)]
    fn deserialize(
        scope: &mut ValueScope<'scope>,
        value: Value<'scope>,
    ) -> Result<Self, TypeError> {
        let value = Number::try_from(value)
            .map_err(|_| create_type_error("Value not a f64", scope, &value))?;
        Ok(value.value())
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use crate::{initialize_with_defaults, traits::DeserializeOwned, Runtime, RuntimeOptions};

    fn test_from<STATE, SOURCE, T>(runtime: &mut Runtime<STATE>, source: SOURCE, expected: T)
    where
        SOURCE: AsRef<str>,
        T: DeserializeOwned + Eq + Debug,
    {
        let val: T = runtime.execute(source).expect("Can't execute code");
        assert_eq!(val, expected);
    }

    fn test_f32<STATE, SOURCE>(runtime: &mut Runtime<STATE>, source: SOURCE, expected: f32)
    where
        SOURCE: AsRef<str>,
    {
        let val: f32 = runtime.execute(source).expect("Can't execute code");
        assert!((val - expected).abs() < f32::EPSILON);
    }

    fn test_f64<STATE, SOURCE>(runtime: &mut Runtime<STATE>, source: SOURCE, expected: f64)
    where
        SOURCE: AsRef<str>,
    {
        let val: f64 = runtime.execute(source).expect("Can't execute code");
        assert!((val - expected).abs() < f64::EPSILON,);
    }

    #[test]
    fn from_value_for_unit() {
        initialize_with_defaults();
        let r = &mut Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        test_from(r, "0", ());
        test_from(r, "1", ());
    }

    #[test]
    fn from_value_for_bool() {
        initialize_with_defaults();
        let r = &mut Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        test_from(r, "false", false);
        test_from(r, "true", true);
    }

    #[test]
    fn from_value_for_string() {
        initialize_with_defaults();
        let r = &mut Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        test_from(r, "'A string'", "A string".to_string());
        test_from(r, "1", "1".to_string());
        test_from(r, "false", "false".to_string());
    }

    #[test]
    fn from_value_for_integer() {
        initialize_with_defaults();
        let r = &mut Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        test_from(r, i8::MIN.to_string(), i8::MIN);
        test_from(r, i8::MAX.to_string(), i8::MAX);
        test_from(r, i16::MIN.to_string(), i16::MIN);
        test_from(r, i16::MAX.to_string(), i16::MAX);
        test_from(r, i32::MIN.to_string(), i32::MIN);
        test_from(r, i32::MAX.to_string(), i32::MAX);

        test_from(r, i32::MIN.to_string(), i32::MIN as i64);
        test_from(r, i32::MAX.to_string(), i32::MAX as i64);

        test_from(r, "-9223372036854775808n", i64::MIN);
        test_from(r, "9223372036854775807n", i64::MAX);

        test_from(r, u8::MIN.to_string(), u8::MIN);
        test_from(r, u8::MAX.to_string(), u8::MAX);
        test_from(r, u16::MIN.to_string(), u16::MIN);
        test_from(r, u16::MAX.to_string(), u16::MAX);
        test_from(r, u32::MIN.to_string(), u32::MIN);
        test_from(r, u32::MAX.to_string(), u32::MAX);
        test_from(r, u64::MIN.to_string(), u64::MIN);
        test_from(r, "18446744073709551615n", u64::MAX);
    }

    #[test]
    fn from_value_for_float() {
        initialize_with_defaults();
        let r = &mut Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        test_f32(r, "0.0", 0.0f32);
        test_f32(r, f32::MIN.to_string(), f32::MIN);
        test_f32(r, f32::MAX.to_string(), f32::MAX);

        test_f64(r, "0.0", 0.0f64);
        test_f64(r, f64::MIN.to_string(), f64::MIN);
        test_f64(r, f64::MAX.to_string(), f64::MAX);
    }
}
