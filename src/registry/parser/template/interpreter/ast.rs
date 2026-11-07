#[derive(Debug, Clone)]
pub enum Expr {
    Path(Vec<String>),
    Str(String),

    Call {
        name: String,
        args: Vec<Expr>,
    },

    Pipeline {
        base: Box<Expr>,
        filters: Vec<Filter>,
    },
}

#[derive(Debug, Clone)]
pub struct Filter {
    pub name: String,
    pub args: Vec<Expr>,
}
