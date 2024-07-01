use chrono::{FixedOffset};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::fmt;
use serde::de::{SeqAccess, Visitor};

#[derive(Debug, Clone, Deserialize)]
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
            deserialize_with = "try_i64_from_str")]
        value: i64,
    },
    Float {
        #[serde(
            rename = "_value",
            deserialize_with = "try_f64_from_str")]
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
    ZoneDateTime {
        #[serde(
            rename = "_value",
            deserialize_with = "try_zdt_from_str"
        )]
        value: chrono::DateTime<FixedOffset>,
    },
    DateTime {
        #[serde(
        rename = "_value",
        deserialize_with = "try_dt_from_str",
        )]
        value: chrono::NaiveDateTime,
    },
    Time {
        #[serde(
        rename = "_value",
        deserialize_with = "try_time_from_str",
        )]
        value: chrono::NaiveTime,
    },
    Date {
        #[serde(
        rename = "_value",
        deserialize_with = "try_date_from_str",
        )]
        value: chrono::NaiveDate,
    },
    Duration {
        #[serde(
        rename = "_value",
        deserialize_with = "try_duration_from_str",
        )]
        value: chrono::TimeDelta,
    },
    Node {
        #[serde(rename = "_value")]
        value: Node,
    },
    Relationship {
        #[serde(rename = "_value")]
        value: Relationship,
    },
    Path {
        #[serde(
            rename = "_value",
            deserialize_with = "try_path",)]
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

fn try_path<'de, D>(deserializer: D) -> Result<Path, D::Error>
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
    use super::*;
    use chrono::Timelike;

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
        let test = "{ \"$type\":\"ZoneDateTime\", \"_value\": \"2012-01-01T12:00:00[+02:00]\" }";
        let inputs = serde_json::Deserializer::from_str(test).into_iter::<Body>();
        for input in inputs {
            match *input.unwrap().body {
                Val::ZoneDateTime { value } => {
                    assert_eq!(12u32, value.hour());
                }
                _ => panic!("test fail"),
            };
        }
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
                let labels: &[String] = &*value.labels;
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

