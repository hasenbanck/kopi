use v8::NewStringType;

use crate::{
    error::TypeError,
    traits::Serialize,
    value::{BigInt, Boolean, Integer, Number, Primitive, String, Value, ValueScope},
};

const MAX_SAFE_INTEGER: i64 = 2i64.pow(53) - 1i64;
const MIN_SAFE_INTEGER: i64 = -(2i64.pow(53) - 1i64);

impl Serialize for () {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(Primitive::new_undefined(scope).into())
    }

    const DEFINED_RETURN_VALUE: bool = false;
}

impl Serialize for bool {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(Boolean::new(scope, self).into())
    }
}

impl Serialize for char {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(String::new(scope, self.to_string(), NewStringType::Normal).into())
    }
}

impl Serialize for i8 {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(Integer::new_from_i32(scope, i32::from(self)).into())
    }
}

impl Serialize for i16 {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(Integer::new_from_i32(scope, i32::from(self)).into())
    }
}

impl Serialize for i32 {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(Integer::new_from_i32(scope, self).into())
    }
}

impl Serialize for i64 {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        if self > MAX_SAFE_INTEGER || self < MIN_SAFE_INTEGER {
            Ok(BigInt::new_from_i64(scope, self).into())
        } else if self > i32::MAX as i64 || self < i32::MIN as i64 {
            Ok(Number::new(scope, self as f64).into())
        } else {
            Ok(Integer::new_from_i32(scope, self as i32).into())
        }
    }
}

impl Serialize for u8 {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(Integer::new_from_u32(scope, u32::from(self)).into())
    }
}

impl Serialize for u16 {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(Integer::new_from_u32(scope, u32::from(self)).into())
    }
}

impl Serialize for u32 {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(Integer::new_from_u32(scope, self).into())
    }
}

impl Serialize for u64 {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        if self > MAX_SAFE_INTEGER as u64 {
            Ok(BigInt::new_from_u64(scope, self).into())
        } else if self > u32::MAX as u64 {
            Ok(Number::new(scope, self as f64).into())
        } else {
            Ok(Integer::new_from_u32(scope, self as u32).into())
        }
    }
}

impl Serialize for f32 {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(Number::new(scope, f64::from(self)).into())
    }
}

impl Serialize for f64 {
    #[inline(always)]
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(Number::new(scope, self).into())
    }
}

impl Serialize for std::string::String {
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(String::new(scope, self.as_str(), NewStringType::Normal).into())
    }
}

impl Serialize for &str {
    fn serialize<'scope>(self, scope: &mut ValueScope<'scope>) -> Result<Value<'scope>, TypeError> {
        Ok(String::new(scope, self, NewStringType::Normal).into())
    }
}

#[cfg(test)]
mod test {
    use super::{MAX_SAFE_INTEGER, MIN_SAFE_INTEGER};
    use crate::{
        initialize_with_defaults, traits::Serialize, Extension, FunctionArguments, Runtime,
        RuntimeOptions,
    };

    pub fn test<F, A, R>(expected_type: &str, expected_value: &str, function: F)
    where
        F: 'static + Send + Sync + Fn(A) -> R,
        A: for<'s> FunctionArguments<'s, F, R>,
        R: Serialize,
    {
        initialize_with_defaults();
        let mut extension = Extension::new(None);
        extension.add_function("test", function);

        let mut r = Runtime::new(
            RuntimeOptions {
                extensions: vec![extension],
                ..Default::default()
            },
            (),
        )
        .expect("Can't create runtime");

        let type_ok: bool = r
            .execute(&format!("let x = test(); typeof x === '{}'", expected_type))
            .expect("Can't execute evaluation code");
        assert!(type_ok);

        let value_ok: bool = r
            .execute(&format!("x = test(); x === {}", expected_value))
            .expect("Can't execute evaluation code");
        assert!(value_ok);
    }

    #[test]
    fn into_value_for_unit() {
        test("undefined", "undefined", |()| ());

        test("boolean", "true", |()| true);
        test("boolean", "false", |()| false);

        test("number", "-4.5", |()| -4.5f32);
        test("number", "9.0", |()| 9.0f32);
        test("number", "-33.33", |()| -33.33f64);
        test("number", "66.66", |()| 66.66f64);

        test("number", "-128", |()| i8::MIN);
        test("number", "127", |()| i8::MAX);
        test("number", "-32768", |()| i16::MIN);
        test("number", "32767", |()| i16::MAX);
        test("number", "-2147483648", |()| i32::MIN);
        test("number", "2147483647", |()| i32::MAX);
        test("number", "-2147483648", |()| i32::MIN as i64);
        test("number", "2147483647", |()| i32::MAX as i64);
        test("number", "-9007199254740991", |()| MIN_SAFE_INTEGER);
        test("number", "9007199254740991", |()| MAX_SAFE_INTEGER);
        test("bigint", "-9223372036854775808n", |()| i64::MIN);
        test("bigint", "9223372036854775807n", |()| i64::MAX);

        test("number", "0", |()| u8::MIN);
        test("number", "255", |()| u8::MAX);
        test("number", "0", |()| u16::MIN);
        test("number", "65535", |()| u16::MAX);
        test("number", "0", |()| u32::MIN);
        test("number", "2147483647", |()| i32::MAX as u32);
        test("number", "4294967295", |()| u32::MAX);
        test("number", "0", |()| u64::MIN);
        test("bigint", "9223372036854775807n", |()| i64::MAX as u64);
        test("bigint", "18446744073709551615n", |()| u64::MAX);
    }

    #[test]
    fn safe_integer() {
        assert_eq!(MIN_SAFE_INTEGER, -9007199254740991);
        assert_eq!(MAX_SAFE_INTEGER, 9007199254740991);
    }
}
