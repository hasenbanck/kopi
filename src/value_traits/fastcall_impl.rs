macro_rules! fastcall_argument {
    ($value_type:ty, $v8_type:ident) => {
        impl super::FastcallArgument for $value_type {
            type Value = $value_type;

            #[inline(always)]
            fn v8_type() -> crate::v8::fast_api::Type {
                v8::fast_api::Type::$v8_type
            }
        }
    };
}

// TODO V8 also supports:
//      * pointer to an embedder type
//      * JavaScript array of primitive types

fastcall_argument!(bool, Bool);
fastcall_argument!(i32, Int32);
fastcall_argument!(u32, Uint32);
fastcall_argument!(f32, Float32);
fastcall_argument!(f64, Float64);

macro_rules! fastcall_return_value {
    ($value_type:ty, $c_type:ident) => {
        impl super::FastcallReturnValue for $value_type {
            type Value = $value_type;

            #[inline(always)]
            fn c_type() -> crate::v8::fast_api::CType {
                v8::fast_api::CType::$c_type
            }
        }
    };
}

fastcall_return_value!(bool, Bool);
fastcall_return_value!(i32, Int32);
fastcall_return_value!(u32, Uint32);
fastcall_return_value!(f32, Float32);
fastcall_return_value!(f64, Float64);

macro_rules! fastcall_sealed {
    ($value_type:ty) => {
        impl super::private::Sealed for $value_type {}
    };
}

fastcall_sealed!(());
fastcall_sealed!(bool);
fastcall_sealed!(i32);
fastcall_sealed!(u32);
fastcall_sealed!(f32);
fastcall_sealed!(f64);
