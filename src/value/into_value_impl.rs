use crate::{error::TypeError, value::ValueBuilder, IntoValue, Value};

impl IntoValue for () {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.undefined())
    }

    fn is_undefined() -> bool {
        true
    }
}

impl IntoValue for bool {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.boolean(self))
    }
}

impl IntoValue for i8 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.integer(i32::from(self)))
    }
}

impl IntoValue for i16 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.integer(i32::from(self)))
    }
}

impl IntoValue for i32 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.integer(self))
    }
}

impl IntoValue for i64 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        if self > i32::MAX as i64 || self < i32::MIN as i64 {
            Ok(value_builder.bigint_from_i64(self))
        } else {
            Ok(value_builder.integer(self as i32))
        }
    }
}

impl IntoValue for u8 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.integer(self as i32))
    }
}

impl IntoValue for u16 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.integer(self as i32))
    }
}

impl IntoValue for u32 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        if self > i32::MAX as u32 {
            Ok(value_builder.bigint_from_u64(self as u64))
        } else {
            Ok(value_builder.integer(self as i32))
        }
    }
}

impl IntoValue for u64 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        if self > i32::MAX as u64 {
            Ok(value_builder.bigint_from_u64(self))
        } else {
            Ok(value_builder.integer(self as i32))
        }
    }
}

impl IntoValue for f32 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.float(self as f64))
    }
}

impl IntoValue for f64 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.float(self))
    }
}

#[cfg(test)]
mod test {
    use crate::{
        initialize_v8, Extension, FunctionArguments, InitializationOptions, IntoValue, Runtime,
        RuntimeOptions,
    };

    pub fn test<F, A, R>(expected_type: &str, expected_value: &str, function: F)
    where
        F: 'static + Send + Sync + Fn(A) -> R,
        A: FunctionArguments<F, R>,
        R: IntoValue,
    {
        initialize_v8(InitializationOptions::default());
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
        test("bigint", "-9223372036854775808n", |()| i64::MIN);
        test("bigint", "9223372036854775807n", |()| i64::MAX);

        test("number", "0", |()| u8::MIN);
        test("number", "255", |()| u8::MAX);
        test("number", "0", |()| u16::MIN);
        test("number", "65535", |()| u16::MAX);
        test("number", "0", |()| u32::MIN);
        test("number", "2147483647", |()| i32::MAX as u32);
        test("bigint", "4294967295n", |()| u32::MAX);
        test("number", "0", |()| u64::MIN);
        test("bigint", "9223372036854775807n", |()| i64::MAX as u64);
        test("bigint", "18446744073709551615n", |()| u64::MAX);
    }
}
