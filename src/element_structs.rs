use element::Element;
use reqwest;

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct SelectedResponse {
    pub value: bool,
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ElementResponse {
    #[serde(rename = "sessionId")]
    session_id: String,
    status: i32,
    value: ElemValue,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct ElementsResponse {
    #[serde(rename = "sessionId")]
    session_id: Option<String>,
    status: i32,
    value: Vec<ElemValue>,
}

#[derive(Debug)]
struct ElemValue {
    // #[serde(rename = "ELEMENT")]
    element_id: String,
}

impl ElemValue {
    pub fn new(element_id: String) -> ElemValue {
        ElemValue { element_id: element_id }
    }
}

use std::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

impl<'de> Deserialize<'de> for ElemValue {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        enum Field { ElementId };

        impl<'de> Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Field, D::Error>
            where
                D: Deserializer<'de>,
            {
                struct FieldVisitor;

                impl<'de> Visitor<'de> for FieldVisitor {
                    type Value = Field;

                    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                        formatter.write_str("`element_id`")
                    }

                    fn visit_str<E>(self, value: &str) -> Result<Field, E>
                    where
                        E: de::Error,
                    {
                        match value {
                            "element_id" => Ok(Field::ElementId),
                            _ => Ok(Field::ElementId),
                        }
                    }
                }

                deserializer.deserialize_identifier(FieldVisitor)
            }
        }

        struct ElemValueVisitor;

        impl<'de> Visitor<'de> for ElemValueVisitor {
            type Value = ElemValue;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("struct ElemValue")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<ElemValue, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let element_id = seq.next_element()?
                    .ok_or_else(|| de::Error::invalid_length(0, &self))?;
                Ok(ElemValue::new(element_id))
            }

            fn visit_map<V>(self, mut map: V) -> Result<ElemValue, V::Error>
            where
                V: MapAccess<'de>,
            {
                let mut element_id = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Field::ElementId => {
                            if element_id.is_some() {
                                return Err(de::Error::duplicate_field("element_id"));
                            }
                            element_id = Some(map.next_value()?);
                        }
                    }
                }
                let element_id = element_id.ok_or_else(|| de::Error::missing_field("element_id"))?;
                Ok(ElemValue::new(element_id))
            }
        }

        const FIELDS: &'static [&'static str] = &["element_id"];
        deserializer.deserialize_struct("ElemValue", FIELDS, ElemValueVisitor)
    }
}


impl<'a> ElementResponse {
    pub fn parse_into_element(self, client: &'a reqwest::Client) -> Element<'a> {
        Element::new(self.value.element_id, self.session_id, client)
    }
}

impl<'a> ElementsResponse {
    pub fn parse_into_elements(self, client: &'a reqwest::Client, session_id:String) -> Vec<Element<'a>> {
        // let session_id = self.session_id;
        self.value
            .into_iter()
            .map(|value| Element::new(value.element_id, session_id.clone(), client))
            .collect()
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct AttributeResponse {
    #[serde(rename = "sessionId")]
    session_id: Option<String>,
    pub value: String,
}

#[derive(Serialize)]
pub struct ValueRequest<'a> {
    value: Vec<&'a str>,
}

impl<'a> ValueRequest<'a> {
    pub fn new(text: &'a str) -> ValueRequest<'a> {
        ValueRequest { value: vec![text] }
    }
}

#[allow(dead_code)]
#[derive(Deserialize)]
pub struct ExecuteScriptResponse<T> {
    #[serde(rename = "sessionId")]
    session_id: String,
    status: i32,
    pub value: T,
}
