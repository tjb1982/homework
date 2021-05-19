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
        Err(de::Error::custom(
            "this error is intended to be discarded; it is provided
                only as a mechanism to ignore irrelevant values (since this
                implementation only provides the field names of `struct` types)"
        ))
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


pub mod date_format {
    use serde::{self, Serializer, Deserializer, Deserialize};
    use chrono::NaiveDate;

    pub const FORMAT: &'static str = "%-m/%-d/%Y";

    pub fn serialize<S> (
        date: &Option<NaiveDate>,
        serializer: S
    ) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        let s = match date {
            Some(date) => date.format(FORMAT).to_string(),
            None => String::new()
        };
        serializer.serialize_str(s.as_str())
    }

    pub fn deserialize<'de, D>(
        deserializer: D
    ) -> Result<Option<NaiveDate>, D::Error>
        where D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        let d = match NaiveDate::parse_from_str(s.as_str(), FORMAT) {
            Ok(d) => Some(d),
            Err(e) if e.to_string().eq("premature end of input") => {
                log::trace!("{}: missing optional field", e);
                None
            },
            Err(e) => {
                log::error!("{}", e);
                None
            }
        };
        
        Ok(d)
    }
}