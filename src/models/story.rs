use serde::{Serialize, Deserialize};
use smallvec::SmallVec;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Story {
    pub id: String,
    pub title: String,
    pub description: String,
    pub chapters: Vec<Chapter>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Chapter {
    pub id: String,
    pub title: String,
    pub paragraphs: Vec<Paragraph>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Paragraph {
    pub id: String,
    pub content: String,
    pub choices: Vec<Choice>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Choice {
    pub caption: String,
    pub action: Action,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Action {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    pub to: SmallVec<[String; 4]>,
}

impl<'de> serde::Deserialize<'de> for Action {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum ToField {
            Multiple(Vec<String>),
            Single(String),
        }
        
        #[derive(Deserialize)]
        struct Helper {
            #[serde(rename = "type")]
            type_: String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            key: Option<String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            value: Option<serde_json::Value>,
            to: ToField,
        }
        
        let helper = Helper::deserialize(deserializer)?;
        let to: SmallVec<[String; 4]> = match helper.to {
            ToField::Multiple(vec) => SmallVec::from_vec(vec),
            ToField::Single(s) => {
                if s.is_empty() {
                    SmallVec::new()
                } else {
                    let mut v = SmallVec::new();
                    v.push(s);
                    v
                }
            }
        };
        
        Ok(Action {
            type_: helper.type_,
            key: helper.key,
            value: helper.value,
            to,
        })
    }
} 