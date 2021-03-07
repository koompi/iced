use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone, PartialEq, Default)]
pub struct TableColumn {
    pub name: String,
    pub label: Option<String>,
    pub short_name: Option<String>,
    pub order: TableOrder,
}

impl Display for TableColumn {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.label.as_ref().unwrap_or(&self.name))
    }
}

impl TableColumn {
    pub fn formatted_sortable_column(&self) -> String {
        if let Some(label) = &self.label {
            match self.order {
                TableOrder::Unordered => label.to_owned(),
                TableOrder::Ascending => format!("{} ▲", label),
                TableOrder::Descending => format!("{} ▼", label),
            }
        } else {
            self.name.to_owned()
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TableOptions {
    pub orderable: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TableOrder {
    Unordered = 0,
    Ascending = 1,
    Descending = 2,
}

impl Default for TableOrder {
    fn default() -> Self {
        Self::Unordered
    }
}

impl TableOrder {
    pub fn toggle(&self) -> Self {
        use TableOrder::*;
        match *self {
            Unordered => Ascending,
            Ascending => Descending,
            Descending => Unordered,
        }
    }
}
