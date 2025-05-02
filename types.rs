/// Represents C types in the compiler
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Char,
    Int,
    Ptr(Box<Type>),
    Void,
}

impl Type {
    /// Gets the size of the type in bytes
    pub fn size(&self) -> usize {
        match self {
            Type::Char => 1,
            Type::Int => 8,
            Type::Ptr(_) => 8,
            Type::Void => 0,
        }
    }

    /// Checks if the type is a pointer
    pub fn is_ptr(&self) -> bool {
        matches!(self, Type::Ptr(_))
    }

    /// Gets the type this pointer points to
    pub fn deref(&self) -> Option<Type> {
        match self {
            Type::Ptr(inner) => Some(*inner.clone()),
            _ => None,
        }
    }

    /// Creates a pointer to this type
    pub fn ptr_to(&self) -> Type {
        Type::Ptr(Box::new(self.clone()))
    }
}
