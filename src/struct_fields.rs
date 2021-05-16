use serde::{Deserializer, de, forward_to_deserialize_any};

pub trait StructFieldDeserialize {
    fn struct_fields() -> &'static[&'static str];
}

pub struct StructFieldsDeserializer<'a> {
    pub fields: &'a mut Option<&'static [&'static str]>,
}

impl<'de> Deserializer<'de> for StructFieldsDeserializer<'de> {
    type Error = serde::de::value::Error;

    fn deserialize_any<V>(self, _visitor: V) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>
    {
        Err(de::Error::custom("please disregard this error"))
    }

    fn deserialize_struct<V>(
        self,
        _name: &'static str,
        fields: &'static [&'static str],
        visitor: V
    ) -> Result<V::Value, Self::Error>
        where V: de::Visitor<'de>
    {
        *self.fields = Some(fields);
        self.deserialize_any(visitor)
    }

    forward_to_deserialize_any! {
        bool i8 i16 i32 i64 u8 u16 u32 u64 f32 f64 char str string bytes
        byte_buf option unit unit_struct newtype_struct seq tuple
        tuple_struct map enum identifier ignored_any
    }
}
