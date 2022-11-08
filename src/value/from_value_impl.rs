use crate::{
    error::{create_type_error, TypeError},
    FromValue, Value,
};

impl FromValue for () {
    type Value = ();

    #[inline(always)]
    fn from_v8(_value: Value) -> Result<Self::Value, TypeError> {
        Ok(())
    }
}

impl FromValue for bool {
    type Value = bool;

    #[inline(always)]
    fn from_v8(mut value: Value) -> Result<Self::Value, TypeError> {
        Ok(value.to_boolean())
    }
}

impl FromValue for String {
    type Value = String;

    #[inline(always)]
    fn from_v8(mut value: Value) -> Result<Self::Value, TypeError> {
        Ok(value.to_string())
    }
}

impl FromValue for i8 {
    type Value = i8;

    #[inline(always)]
    fn from_v8(mut value: Value) -> Result<Self::Value, TypeError> {
        if let Some(val) = value.try_integer() {
            let val = i8::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i8", &mut value))?;
            Ok(val)
        } else if let Some(val) = value.try_int32() {
            let val = i8::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i8", &mut value))?;
            Ok(val)
        } else if let Some(val) = value.try_uint32() {
            let val = i8::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i8", &mut value))?;
            Ok(val)
        } else if let Some((val, lossless)) = value.try_bigint_as_i64() {
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an i8",
                    &mut value,
                ));
            }
            let val = i8::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i8", &mut value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an i8",
                &mut value,
            ))
        }
    }
}

impl FromValue for i16 {
    type Value = i16;

    #[inline(always)]
    fn from_v8(mut value: Value) -> Result<Self::Value, TypeError> {
        if let Some(val) = value.try_integer() {
            let val = i16::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i16", &mut value))?;
            Ok(val)
        } else if let Some(val) = value.try_int32() {
            let val = i16::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i16", &mut value))?;
            Ok(val)
        } else if let Some(val) = value.try_uint32() {
            let val = i16::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i16", &mut value))?;
            Ok(val)
        } else if let Some((val, lossless)) = value.try_bigint_as_i64() {
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an i16",
                    &mut value,
                ));
            }
            let val = i16::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i16", &mut value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an i16",
                &mut value,
            ))
        }
    }
}

impl FromValue for i32 {
    type Value = i32;

    #[inline(always)]
    fn from_v8(mut value: Value) -> Result<Self::Value, TypeError> {
        if let Some(val) = value.try_integer() {
            let val = i32::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i32", &mut value))?;
            Ok(val)
        } else if let Some(val) = value.try_int32() {
            Ok(val)
        } else if let Some(val) = value.try_uint32() {
            let val = i32::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i32", &mut value))?;
            Ok(val)
        } else if let Some((val, lossless)) = value.try_bigint_as_i64() {
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an i32",
                    &mut value,
                ));
            }
            let val = i32::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i32", &mut value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an i32",
                &mut value,
            ))
        }
    }
}

impl FromValue for i64 {
    type Value = i64;

    #[inline(always)]
    fn from_v8(mut value: Value) -> Result<Self::Value, TypeError> {
        if let Some(val) = value.try_integer() {
            Ok(val)
        } else if let Some((val, lossless)) = value.try_bigint_as_i64() {
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an i64",
                    &mut value,
                ));
            }
            Ok(val)
        } else if let Some(val) = value.try_int32() {
            let val = i64::from(val);
            Ok(val)
        } else if let Some(val) = value.try_uint32() {
            let val = i64::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an i64", &mut value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an i64",
                &mut value,
            ))
        }
    }
}

impl FromValue for u8 {
    type Value = u8;

    #[inline(always)]
    fn from_v8(mut value: Value) -> Result<Self::Value, TypeError> {
        if let Some(val) = value.try_integer() {
            let val = u8::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u8", &mut value))?;
            Ok(val)
        } else if let Some(val) = value.try_uint32() {
            let val = u8::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u8", &mut value))?;
            Ok(val)
        } else if let Some(val) = value.try_int32() {
            let val = u8::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u8", &mut value))?;
            Ok(val)
        } else if let Some((val, lossless)) = value.try_bigint_as_u64() {
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an u8",
                    &mut value,
                ));
            }
            let val = u8::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u8", &mut value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an u8",
                &mut value,
            ))
        }
    }
}

impl FromValue for u16 {
    type Value = u16;

    #[inline(always)]
    fn from_v8(mut value: Value) -> Result<Self::Value, TypeError> {
        if let Some(val) = value.try_integer() {
            let val = u16::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u16", &mut value))?;
            Ok(val)
        } else if let Some(val) = value.try_int32() {
            let val = u16::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u16", &mut value))?;
            Ok(val)
        } else if let Some(val) = value.try_uint32() {
            let val = u16::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u16", &mut value))?;
            Ok(val)
        } else if let Some((val, lossless)) = value.try_bigint_as_u64() {
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an u16",
                    &mut value,
                ));
            }
            let val = u16::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u16", &mut value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an u16",
                &mut value,
            ))
        }
    }
}

impl FromValue for u32 {
    type Value = u32;

    #[inline(always)]
    fn from_v8(mut value: Value) -> Result<Self::Value, TypeError> {
        if let Some(val) = value.try_integer() {
            let val = u32::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u32", &mut value))?;
            Ok(val)
        } else if let Some(val) = value.try_uint32() {
            Ok(val)
        } else if let Some(val) = value.try_int32() {
            let val = u32::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u32", &mut value))?;
            Ok(val)
        } else if let Some((val, lossless)) = value.try_bigint_as_u64() {
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an u32",
                    &mut value,
                ));
            }
            let val = u32::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u32", &mut value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an u32",
                &mut value,
            ))
        }
    }
}

impl FromValue for u64 {
    type Value = u64;

    #[inline(always)]
    fn from_v8(mut value: Value) -> Result<Self::Value, TypeError> {
        if let Some(val) = value.try_integer() {
            let val = u64::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u64", &mut value))?;
            Ok(val)
        } else if let Some((val, lossless)) = value.try_bigint_as_u64() {
            if !lossless {
                return Err(create_type_error(
                    "Value not in range for an u64",
                    &mut value,
                ));
            }
            Ok(val)
        } else if let Some(val) = value.try_uint32() {
            let val = u64::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u64", &mut value))?;
            Ok(val)
        } else if let Some(val) = value.try_int32() {
            let val = u64::try_from(val)
                .map_err(|_| create_type_error("Value not in range for an u64", &mut value))?;
            Ok(val)
        } else {
            Err(create_type_error(
                "Value can't be converted to an u16",
                &mut value,
            ))
        }
    }
}

impl FromValue for f32 {
    type Value = f32;

    #[inline(always)]
    fn from_v8(mut value: Value) -> Result<Self::Value, TypeError> {
        let value = value
            .try_float()
            .ok_or_else(|| create_type_error("Value not a f32", &mut value))?;
        Ok(value as f32)
    }
}

impl FromValue for f64 {
    type Value = f64;

    #[inline(always)]
    fn from_v8(mut value: Value) -> Result<Self::Value, TypeError> {
        let value = value
            .try_float()
            .ok_or_else(|| create_type_error("Value not a f64", &mut value))?;
        Ok(value)
    }
}

#[cfg(test)]
mod test {
    use std::fmt::Debug;

    use crate::{initialize_v8, value::FromValue, InitializationOptions, Runtime, RuntimeOptions};

    fn test_from<STATE, SOURCE, T>(runtime: &mut Runtime<STATE>, source: SOURCE, expected: T)
    where
        SOURCE: AsRef<str>,
        T: FromValue<Value = T> + Eq + Debug,
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
        initialize_v8(InitializationOptions::default());
        let r = &mut Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        test_from(r, "0", ());
        test_from(r, "1", ());
    }

    #[test]
    fn from_value_for_bool() {
        initialize_v8(InitializationOptions::default());
        let r = &mut Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        test_from(r, "0", false);
        test_from(r, "1", true);
        test_from(r, "false", false);
        test_from(r, "true", true);
        test_from(r, "'a_string'", true);
    }

    #[test]
    fn from_value_for_string() {
        initialize_v8(InitializationOptions::default());
        let r = &mut Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        test_from(r, "'A string'", "A string".to_string());
        test_from(r, "1", "1".to_string());
        test_from(r, "false", "false".to_string());
    }

    #[test]
    fn from_value_for_integer() {
        initialize_v8(InitializationOptions::default());
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
        initialize_v8(InitializationOptions::default());
        let r = &mut Runtime::new(RuntimeOptions::default(), ()).expect("Can't create runtime");

        test_f32(r, "0.0", 0.0f32);
        test_f32(r, f32::MIN.to_string(), f32::MIN);
        test_f32(r, f32::MAX.to_string(), f32::MAX);

        test_f64(r, "0.0", 0.0f64);
        test_f64(r, f64::MIN.to_string(), f64::MIN);
        test_f64(r, f64::MAX.to_string(), f64::MAX);
    }
}
