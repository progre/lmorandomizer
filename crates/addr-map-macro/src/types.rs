use crate::util::to_pascal_case;

#[derive(Clone)]
pub struct Label {
    pub offset: usize,
    pub name: String,
    pub comment: Option<String>,
}

#[derive(Clone)]
pub struct Static {
    pub offset: usize,
    pub name: String,
    pub ty: syn::Type,
    pub comment: Option<String>,
}

#[derive(Clone)]
pub struct Function {
    pub offset: usize,
    pub name: String,
    pub ty: syn::TypeBareFn,
    pub comment: Option<String>,
}

#[derive(Clone)]
pub struct StaticFnPtr {
    pub offset: usize,
    pub name: String,
    pub fn_ty: syn::TypeBareFn,
    pub comment: Option<String>,
}

impl StaticFnPtr {
    pub fn to_type_name(&self) -> String {
        format!("{}Fn", to_pascal_case(&self.name))
    }
}

#[derive(Clone)]
pub enum SimpleEntry {
    Label(Label),
    Static(Static),
    StaticFnPtr(StaticFnPtr),
    Function(Function),
}

impl SimpleEntry {
    pub fn offset(&self) -> usize {
        match self {
            SimpleEntry::Label(Label { offset, .. })
            | SimpleEntry::Static(Static { offset, .. })
            | SimpleEntry::StaticFnPtr(StaticFnPtr { offset, .. })
            | SimpleEntry::Function(Function { offset, .. }) => *offset,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            SimpleEntry::Label(Label { name, .. })
            | SimpleEntry::Static(Static { name, .. })
            | SimpleEntry::StaticFnPtr(StaticFnPtr { name, .. })
            | SimpleEntry::Function(Function { name, .. }) => name,
        }
    }

    pub fn comment(&self) -> Option<&str> {
        match self {
            SimpleEntry::Label(Label { comment, .. })
            | SimpleEntry::Static(Static { comment, .. })
            | SimpleEntry::StaticFnPtr(StaticFnPtr { comment, .. })
            | SimpleEntry::Function(Function { comment, .. }) => comment.as_deref(),
        }
    }
}

#[derive(Clone)]
pub enum Entry {
    Simple(SimpleEntry),
    Nested {
        entrypoint: Function,
        children: Vec<SimpleEntry>,
    },
}

impl Entry {
    pub fn offset(&self) -> usize {
        match self {
            Entry::Simple(entry) => entry.offset(),
            Entry::Nested { entrypoint, .. } => entrypoint.offset,
        }
    }
}
