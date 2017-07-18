use serde_json;
use errors::*;
use events::*;

#[derive(Clone, Serialize, Deserialize)]
pub struct Document {
    title: String,
    body: Vec<String>,
}

pub struct Store {
    data: Vec<Document>,
    changes: Option<Vec<Change>>,
}

impl Store {
    pub fn new() -> Self {
        Self {
            data: Vec::<Document>::new(),
            changes: None,
        }
    }

    pub fn insert(&mut self, i: &str) -> Result<()> {
        let doc: Document = serde_json::from_str(i)?;
        self.data.push(doc);
        Ok(())
    }

    pub fn take_changes(&mut self) -> Option<Vec<Change>> {
        self.changes.take()
    }

    pub fn snap(&self) -> Box<[Document]> {
        self.data.to_vec().into_boxed_slice()
    }

    pub fn apply(&mut self, f: Fact) {
        use self::Fact::*;
        match f {
            Delete => unimplemented!(),
        }
    }
}
