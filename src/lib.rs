use chrono::{DateTime, FixedOffset, NaiveDate, NaiveDateTime, NaiveTime, TimeDelta};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::collections::HashMap;
use std::fmt;
use serde::de::{SeqAccess, Visitor};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "$type")]
pub enum Val {
    // Null {
    //     #[serde(rename = "_value")]
    //     value: Option<T>,
    // },
    Boolean {
        #[serde(rename = "_value")]
        value: bool,
    },
    Integer {
        #[serde(
            rename = "_value",
            deserialize_with = "try_i64_from_str",
            serialize_with = "try_ser_i64")]
        value: i64,
    },
    Float {
        #[serde(
            rename = "_value",
            deserialize_with = "try_f64_from_str",
            serialize_with = "try_ser_f64")]
        value: f64,
    },
    String {
        #[serde(rename = "_value")]
        value: String,
    },
    ByteArray {
        #[serde(rename = "_value")]
        value: Box<[u8]>,
    },
    Map {
        #[serde(rename = "_value")]
        value: HashMap<String, Box<Val>>,
    },
    List {
        #[serde(rename = "_value")]
        value: Vec<Val>,
    },
    ZonedDateTime {
        #[serde(
            rename = "_value",
            deserialize_with = "try_zdt_from_str",
            serialize_with = "try_ser_zdt"
        )]
        value: chrono::DateTime<FixedOffset>,
    },
    DateTime {
        #[serde(
        rename = "_value",
        deserialize_with = "try_dt_from_str",
        serialize_with = "try_ser_datetime"
        )]
        value: chrono::NaiveDateTime,
    },
    Time {
        #[serde(
        rename = "_value",
        deserialize_with = "try_time_from_str",
        serialize_with = "try_ser_time"
        )]
        value: chrono::NaiveTime,
    },
    Date {
        #[serde(
            rename = "_value",
            deserialize_with = "try_date_from_str",
            serialize_with = "try_ser_date"
        )]
        value: chrono::NaiveDate,
    },
    Duration {
        #[serde(
            rename = "_value",
            deserialize_with = "try_duration_from_str",
            serialize_with = "try_ser_duration"
        )]
        value: chrono::TimeDelta,
    },
    Node {
        #[serde(
            rename = "_value",
            serialize_with = "try_ser_node")]
        value: Node,
    },
    Relationship {
        #[serde(
            rename = "_value",
            serialize_with = "try_ser_rel")]
        value: Relationship,
    },
    Path {
        #[serde(
            rename = "_value",
            deserialize_with = "try_de_path",
            serialize_with = "try_ser_path")]
        value: Path,
    },
}

fn try_i64_from_str<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    let string: String = Deserialize::deserialize(deserializer).unwrap();
    let integer: i64 = string.parse().unwrap();
    Ok(integer)
}

fn try_f64_from_str<'de, D>(deserializer: D) -> Result<f64, D::Error>
where
    D: Deserializer<'de>,
{
    let string: String = Deserialize::deserialize(deserializer).unwrap();
    let float: f64 = string.parse().unwrap();
    Ok(float)
}

fn try_zdt_from_str<'de, D>(deserializer: D) -> Result<chrono::DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(chrono::DateTime::<FixedOffset>::parse_from_str(
        Deserialize::deserialize(deserializer).unwrap(),
        "%Y-%m-%dT%H:%M:%S[%:z]",
    )
    .unwrap())
}

fn try_dt_from_str<'de, D>(deserializer: D) -> Result<chrono::NaiveDateTime, D::Error>
    where
        D: Deserializer<'de>,
{
    Ok(chrono::NaiveDateTime::parse_from_str(
        Deserialize::deserialize(deserializer).unwrap(),
        "%Y-%m-%dT%H:%M:%S",
    ).unwrap())
}

fn try_time_from_str<'de, D>(deserializer: D) -> Result<chrono::NaiveTime, D::Error>
    where
        D: Deserializer<'de>,
{
    Ok(chrono::NaiveTime::parse_from_str(
        Deserialize::deserialize(deserializer).unwrap(),
        "%H:%M:%S",
    ).unwrap())
}
fn try_date_from_str<'de, D>(deserializer: D) -> Result<chrono::NaiveDate, D::Error>
    where
        D: Deserializer<'de>,
{
    Ok(chrono::NaiveDate::parse_from_str(
        Deserialize::deserialize(deserializer).unwrap(),
        "%Y-%m-%d",
    ).unwrap())
}
fn try_duration_from_str<'de, D>(deserializer: D) -> Result<chrono::TimeDelta, D::Error>
    where
        D: Deserializer<'de>,
{
    let _: String = Deserialize::deserialize(deserializer).unwrap();
    Ok(chrono::TimeDelta::new(0,0).unwrap())
}

fn try_de_path<'de, D>(deserializer: D) -> Result<Path, D::Error>
    where
        D: Deserializer<'de>,
{
    struct PathVisitor;
    impl<'de> Visitor<'de> for PathVisitor {
        type Value = Path;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("struct Path")
        }

        fn visit_seq<V>(self, mut seq: V) -> Result<Path, V::Error>
        where
            V: SeqAccess<'de>,
        {
            let mut nodes = Vec::new();
            let mut relationships = Vec::new();

            while let Some(value) = seq.next_element::<Val>()? {
                match value {
                    Val::Node { value } => {
                        nodes.push(value)
                    }
                    Val:: Relationship {value} => {
                        relationships.push(value);
                    },
                    _ => { panic!("Expected Node or Relationship when deserializing Path")}
                }
            }

            Ok(Path { nodes, relationships })
        }
    }

    return deserializer.deserialize_seq(PathVisitor);
}

fn try_ser_i64<S>(int: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&int.to_string())
}

fn try_ser_f64<S>(float: &f64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&float.to_string())
}

fn try_ser_datetime<S>(datetime: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error>
where
S: Serializer,
{
    serializer.serialize_str("todo")
}

fn try_ser_zdt<S>(datetime: &DateTime<FixedOffset>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("todo")
}


fn try_ser_time<S>(time: &NaiveTime, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("todo")
}

fn try_ser_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("todo")
}

fn try_ser_duration<S>(duration: &TimeDelta, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("todo")
}

fn try_ser_node<S>(node: &Node, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("todo")
}

fn try_ser_rel<S>(rel: &Relationship, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("todo")
}

fn try_ser_path<S>(path: &Path, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str("todo")
}




#[derive(Deserialize, Debug, Clone)]
pub struct Node {
    #[serde(rename = "_labels")]
    pub labels: Vec<String>,
    #[serde(rename = "_properties")]
    pub properties: HashMap<String, Val>,
    #[serde(rename = "_element_id")]
    pub element_id: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Relationship {
    #[serde(rename = "_type")]
    pub type_: String,
    #[serde(rename = "_properties")]
    pub properties: HashMap<String, Val>,
    #[serde(rename = "_element_id")]
    pub element_id: String,
    #[serde(rename = "_start_node_element_id")]
    pub start_node_element_id: String,
    #[serde(rename = "_end_node_element_id")]
    pub end_node_element_id: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Path {
    pub nodes: Vec<Node>,
    pub relationships: Vec<Relationship>
}

//
#[derive(Debug, Clone, Deserialize)]
struct Body {
    #[serde(flatten)]
    body: Box<Val>,
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;
    use std::string;
    use super::*;
    use chrono::Timelike;
    use crate::Val::{Boolean, ByteArray, DateTime, Float, Integer, Map, String, ZonedDateTime};
    // #[test]
    // fn null_deserializes() {
    //     let test = "{ \"$type\":\"Null\", \"_value\": null }";
    //     let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
    //     for input in inputs {
    //         match *input.unwrap().body {
    //             Val::Null { value } => {
    //                 assert_eq!(None, value);
    //             }
    //             _ => panic!("test fail"),
    //         };
    //     }
    // }

    #[test]
    fn bool_deserializes() {
        let test = "{ \"$type\":\"Boolean\", \"_value\": true }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Boolean { value } => {
                    assert_eq!(true, value);
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn bool_serializes() {
        let input = Boolean { value: false };
        let output = serde_json::to_string(&input).unwrap();
        assert_eq!(output, "{\"$type\":\"Boolean\",\"_value\":false}");
    }

    #[test]
    fn i64_deserializes() {
        let test = "{ \"$type\":\"Integer\", \"_value\": \"10\" }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Integer { value } => {
                    assert_eq!(10i64, value);
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn i64_serializes() {
        let input = Integer { value: 123 };
        let output = serde_json::to_string(&input).unwrap();
        assert_eq!(output, "{\"$type\":\"Integer\",\"_value\":\"123\"}");
    }

    #[test]
    fn f64_deserializes() {
        let test = "{ \"$type\":\"Float\", \"_value\": \"1.0\" }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Float { value } => {
                    assert_eq!(1.0, value);
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn f64_serializes() {
        let input = Float { value: 1.23 };
        let output = serde_json::to_string(&input).unwrap();
        assert_eq!(output, "{\"$type\":\"Float\",\"_value\":\"1.23\"}");
    }

    #[test]
    fn string_deserializes() {
        let test = "{ \"$type\":\"String\", \"_value\": \"bert\" }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::String { value } => {
                    assert_eq!("bert", value);
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn string_serializes() {
        let input = Val::String { value: "Grant Loves Java!".parse().unwrap() };
        let output = serde_json::to_string(&input).unwrap();
        assert_eq!(output, "{\"$type\":\"String\",\"_value\":\"Grant Loves Java!\"}");
    }

    #[test]
    fn u8_deserializes() {
        let test = "{ \"$type\":\"ByteArray\", \"_value\": [1,2,3,4,255] }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::ByteArray { value } => {
                    let res: &[u8] = &*value;
                    assert_eq!([1u8,2u8,3u8,4u8,255u8], res);
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn u8_serializes() {
        let byte_array = Vec::from([1u8,2u8,3u8,4u8,255u8]);
        let input = ByteArray { value: byte_array.into_boxed_slice() };
        let output = serde_json::to_string(&input).unwrap();
        assert_eq!(output, "{\"$type\":\"ByteArray\",\"_value\":[1,2,3,4,255]}");
    }

    #[test]
    fn map_deserializes() {
        let test = "{ \"$type\":\"Map\", \"_value\": {\"k\": { \"$type\":\"String\", \"_value\": \"bert\" } } }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Map { value } => {
                    let v = value.get("k").unwrap().as_ref().clone();
                    match v {
                        Val::String { value } => {
                            assert_eq!("bert", value);
                        }
                        _ => panic!("test fail"),
                    }
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn map_serializes() {
        let mut map:HashMap<string::String, Box<Val>> = HashMap::new();
        map.insert("k".to_string(), Box::from(String { value: "bert".to_string() }));
        let input = Map { value: map };


        let output = serde_json::to_string(&input).unwrap();
        let test = "{\"$type\":\"Map\",\"_value\":{\"k\":{\"$type\":\"String\",\"_value\":\"bert\"}}}";
        assert_eq!(output, test);
    }

    #[test]
    fn nest_map_deserializes() {
        let test = "{ \"$type\":\"Map\", \"_value\": {\"k\": { \"$type\":\"Map\", \"_value\":  {\"m\": { \"$type\":\"String\", \"_value\": \"bert\" } } } } }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Map { value } => {
                    let v = value.get("k").unwrap().as_ref().clone();
                    match v {
                        Val::Map { value } => {
                            let r = value.get("m").unwrap().as_ref().clone();
                            match r {
                                Val::String { value } => {
                                    assert_eq!("bert", value);
                                }
                                _ => panic!("test fail"),
                            }
                        }
                        _ => panic!("test fail"),
                    }
                }
                _ => panic!("test fail"),
            };
        }
    }
    #[test]
    fn zdt_deserializes() {
        let test = "{ \"$type\":\"ZonedDateTime\", \"_value\": \"2012-01-01T12:00:00[+02:00]\" }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::ZonedDateTime { value } => {
                    assert_eq!(12u32, value.hour());
                }
                _ => panic!("test fail"),
            };
        }
    }
    #[test]
    fn zdt_serializes() {
        let zdt = chrono::DateTime::<FixedOffset>::parse_from_str(
            "2012-01-01T12:00:00[+02:00]",
            "%Y-%m-%dT%H:%M:%S[%:z]",
        ).unwrap();

        let input = ZonedDateTime {value: zdt};

        let output = serde_json::to_string(&input).unwrap();
        assert_eq!(output, "{\"$type\":\"ZonedDateTime\",\"_value\":\"2012-01-01T12:00:00[+02:00])\"}");
    }
    #[test]
    fn dt_deserializes() {
        let test = "{ \"$type\":\"DateTime\", \"_value\": \"2012-01-01T12:00:00\" }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::DateTime { value } => {
                    assert_eq!(12u32, value.hour());
                }
                _ => panic!("test fail"),
            };
        }
    }
    // #[test ]
    // fn duration_deserializes() {
    //     let test = "{ \"$type\":\"Duration\", \"_value\": \"P14DT16H12M\" }";
    //     let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
    //     for input in inputs {
    //         match *input.unwrap().body {
    //             Val::DateTime { value } => {
    //                 assert_eq!(12u32, value.hour());
    //             }
    //             _ => panic!("test fail"),
    //         };
    //     }
    // }
    #[test]
    fn list_deserializes() {
        let test = "{ \"$type\":\"List\", \"_value\": [{ \"$type\":\"Integer\", \"_value\": \"10\" }]}";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::List { value } => {
                    match *value.get(0).unwrap() {
                        Val::Integer {value} => {
                            assert_eq!(value, 10);
                        }
                        _ => panic!("test fail"),
                    }
                }
                _ => panic!("test fail"),
            };
        }
    }
    #[test]
    fn nest_list_deserializes() {
        let test = "{ \"$type\":\"List\", \"_value\": [{ \"$type\":\"List\", \"_value\": [{\"$type\":\"Integer\", \"_value\": \"10\"}] }]}";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::List { value } => {
                    let outer = value.get(0).unwrap();
                    match outer {
                        Val::List { value } => {
                            let inner = value.get(0).unwrap();
                            match inner {
                                Val::Integer {value} => {
                                    assert_eq!(*value, 10);
                                }
                                _ => panic!("test fail"),
                            }
                        }
                        _ => panic!("test fail"),
                    }
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn node_deserializes() {

        let test = "{
                \"$type\": \"Node\",
                \"_value\": {
                    \"_element_id\": \"4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:13\",
                    \"_labels\": [\"Person\"],
                    \"_properties\": {
                        \"name\": {
                            \"$type\": \"String\",
                            \"_value\": \"Richard\"
                        }
                    }
                }
            }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Node { value } => {
                    assert_eq!("4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:13", value.element_id);
                    let labels: &[string::String] = &*value.labels;
                    assert_eq!(labels,value.labels);
                    assert_eq!(1,value.properties.len());

                    match &value.properties.get("name").unwrap() {
                        Val::String { value } => {
                            assert_eq!("Richard", value)
                        }
                        _ => panic!("test fail")
                    }
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn rel_deserializes() {
        let test = "{
                \"$type\": \"Relationship\",
                \"_value\": {
                    \"_element_id\": \"5:ca452f2f-1fbe-4d91-8b67-486b237e24c5:1152921504606846989\",
                    \"_start_node_element_id\": \"4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:13\",
                    \"_end_node_element_id\": \"4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:14\",
                    \"_type\": \"RIDES\",
                    \"_properties\": {
                        \"name\": {
                            \"$type\": \"String\",
                            \"_value\": \"Richard\"
                        }
                    }
                }
            }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Relationship { value } => {
                    assert_eq!("5:ca452f2f-1fbe-4d91-8b67-486b237e24c5:1152921504606846989", value.element_id);
                    assert_eq!("4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:13", value.start_node_element_id);
                    assert_eq!("4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:14", value.end_node_element_id);
                    assert_eq!("RIDES", value.type_);
                    assert_eq!(1, value.properties.len());

                    match &value.properties.get("name").unwrap() {
                        Val::String { value } => {
                            assert_eq!("Richard", value)
                        }
                        _ => panic!("test fail")
                    }
                }
                _ => panic!("test fail"),
            };
        }
    }

    #[test]
    fn path_deserializes() {
        let test = "{
                \"$type\": \"Path\",
                \"_value\": [
                    {
                        \"$type\": \"Node\",
                        \"_value\": {
                            \"_element_id\": \"4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:13\",
                            \"_labels\": [\"Person\"],
                            \"_properties\": {}
                        }
                    },
                    {
                        \"$type\": \"Relationship\",
                        \"_value\": {
                            \"_element_id\": \"5:ca452f2f-1fbe-4d91-8b67-486b237e24c5:1152921504606846989\",
                            \"_start_node_element_id\": \"4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:13\",
                            \"_end_node_element_id\": \"4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:14\",
                            \"_type\": \"RIDES\",
                            \"_properties\": {}
                        }
                    },
                    {
                        \"$type\": \"Node\",
                        \"_value\": {
                            \"_element_id\": \"4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:14\",
                            \"_labels\": [\"Bicycle\"],
                            \"_properties\": {}
                        }
                    }
                ]
            }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::Path { value } => {
                    assert_eq!(2, value.nodes.len());
                    assert_eq!(1, value.relationships.len());

                    assert_eq!(value.nodes.get(0).unwrap().element_id,
                               "4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:13");
                    assert_eq!(value.nodes.get(1).unwrap().element_id,
                               "4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:14");

                    assert_eq!(value.relationships.get(0).unwrap().element_id,
                               "5:ca452f2f-1fbe-4d91-8b67-486b237e24c5:1152921504606846989");
                    assert_eq!(value.relationships.get(0).unwrap().start_node_element_id,
                               "4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:13");
                    assert_eq!(value.relationships.get(0).unwrap().end_node_element_id,
                               "4:ca452f2f-1fbe-4d91-8b67-486b237e24c5:14");
                }
                _ => panic!("test fail"),
            };
        }
    }
}

