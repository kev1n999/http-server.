// Available http methods
#[derive(Debug, PartialEq, Eq)]
pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
    Invalid,
}

#[derive(Debug)]
pub enum AllowedMethods {
    Unique(Method),
    Multiple(Vec<Method>),
}

impl AllowedMethods {
    pub fn contains(&self, method: &Method) -> bool {
        match self {
            Self::Unique(allowed) => allowed == method,
            Self::Multiple(methods) => methods.contains(method),
        }
    }
}
