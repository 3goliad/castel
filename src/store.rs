use errors::*;
use events::*;

enum Datum {
    String,
}

pub type Viewdoc = Vec<usize>;

pub struct Changes;

impl Changes {
    pub fn next(&mut self) -> Option<Change> {
        unimplemented!();
    }
}

pub struct Store {
    docs: Vec<Datum>,
    view: Viewdoc,
    pub changes: Changes,
}

impl Store {
    pub fn new() -> Self {
        Self {
            docs: Vec::<Datum>::new(),
            view: Viewdoc::new(),
            changes: Changes {},
        }
    }

    pub fn apply(&mut self, f: Fact) {
        unimplemented!();
    }

    pub fn view(&self) -> &Viewdoc {
        unimplemented!();
    }
}
