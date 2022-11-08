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

// TODO test
impl IntoValue for i8 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.integer(i32::from(self)))
    }
}

// TODO test
impl IntoValue for i16 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.integer(i32::from(self)))
    }
}

// TODO test
impl IntoValue for i32 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.integer(self))
    }
}

// TODO test
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

// TODO test
impl IntoValue for u8 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.integer(self as i32))
    }
}

// TODO test
impl IntoValue for u16 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.integer(self as i32))
    }
}

// TODO test
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

// TODO test
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

// TODO test
impl IntoValue for f32 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.float(self as f64))
    }
}

// TODO test
impl IntoValue for f64 {
    #[inline(always)]
    fn into_v8<'borrow, 'scope>(
        self,
        value_builder: &'borrow mut ValueBuilder<'borrow, 'scope>,
    ) -> Result<Value<'borrow, 'scope>, TypeError> {
        Ok(value_builder.float(self))
    }
}
