use serde::ser::{
    SerializeMap, SerializeSeq, SerializeStruct, SerializeStructVariant, SerializeTuple,
    SerializeTupleStruct, SerializeTupleVariant, Serializer,
};

use crate::{
    error::TypeError,
    value::{Value, ValueScope},
    Serialize,
};

/// Custom serializer to serialize a Rust type into a engine [`Value`].
pub(crate) struct ValueSerializer<'a, 'scope> {
    pub(crate) scope: &'a mut ValueScope<'scope>,
}

impl<'a, 'scope> Serializer for &'a mut ValueSerializer<'a, 'scope> {
    type Ok = Value<'scope>;
    type Error = TypeError;
    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, v: bool) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_i8(self, v: i8) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_i16(self, v: i16) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_i32(self, v: i32) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_i64(self, v: i64) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_u8(self, v: u8) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_u16(self, v: u16) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_u32(self, v: u32) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_u64(self, v: u64) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_f32(self, v: f32) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_f64(self, v: f64) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_char(self, v: char) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_str(self, v: &str) -> Result<Self::Ok, Self::Error> {
        v.serialize(self.scope)
    }

    fn serialize_bytes(self, _v: &[u8]) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_none(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_some<T: ?Sized>(self, _v: &T) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        todo!()
    }

    fn serialize_unit(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<Self::Ok, Self::Error> {
        todo!()
    }

    fn serialize_newtype_struct<T: ?Sized>(
        self,
        _name: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        todo!()
    }

    fn serialize_newtype_variant<T: ?Sized>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<Self::Ok, Self::Error>
    where
        T: serde::ser::Serialize,
    {
        todo!()
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq, Self::Error> {
        todo!()
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple, Self::Error> {
        todo!()
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct, Self::Error> {
        todo!()
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant, Self::Error> {
        todo!()
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap, Self::Error> {
        todo!()
    }

    fn serialize_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStruct, Self::Error> {
        todo!()
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant, Self::Error> {
        todo!()
    }
}

impl<'a, 'scope> SerializeSeq for &'a mut ValueSerializer<'a, 'scope> {
    type Ok = Value<'scope>;
    type Error = TypeError;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, 'scope> SerializeTuple for &'a mut ValueSerializer<'a, 'scope> {
    type Ok = Value<'scope>;
    type Error = TypeError;

    fn serialize_element<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, 'scope> SerializeTupleStruct for &'a mut ValueSerializer<'a, 'scope> {
    type Ok = Value<'scope>;
    type Error = TypeError;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, 'scope> SerializeTupleVariant for &'a mut ValueSerializer<'a, 'scope> {
    type Ok = Value<'scope>;
    type Error = TypeError;

    fn serialize_field<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, 'scope> SerializeMap for &'a mut ValueSerializer<'a, 'scope> {
    type Ok = Value<'scope>;
    type Error = TypeError;

    fn serialize_key<T: ?Sized>(&mut self, _key: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        todo!()
    }

    fn serialize_value<T: ?Sized>(&mut self, _value: &T) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, 'scope> SerializeStruct for &'a mut ValueSerializer<'a, 'scope> {
    type Ok = Value<'scope>;
    type Error = TypeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}

impl<'a, 'scope> SerializeStructVariant for &'a mut ValueSerializer<'a, 'scope> {
    type Ok = Value<'scope>;
    type Error = TypeError;

    fn serialize_field<T: ?Sized>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<(), Self::Error>
    where
        T: serde::ser::Serialize,
    {
        todo!()
    }

    fn end(self) -> Result<Self::Ok, Self::Error> {
        todo!()
    }
}
