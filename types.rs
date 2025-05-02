#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Type {
    Char,
    Int,
    Ptr(Box<Type>), // Pointer to another type (e.g., `int*`)
    Void,            // For functions with no return type
}

impl Type {
    /// Returns the size (in bytes) of the type.
    pub fn size(&self) -> usize {
        match self {
            Type::Char => 1,
            Type::Int => 8, // C4 uses 8-byte integers (`#define int long long`)
            Type::Ptr(_) => 8, // Pointers are 8 bytes (64-bit)
            Type::Void => 0,
        }
    }

    /// Checks if a type is a pointer (e.g., `int*`, `char****`).
    pub fn is_ptr(&self) -> bool {
        matches!(self, Type::Ptr(_))
    }

    /// Dereferences a pointer type (e.g., `int*` → `int`).
    pub fn deref(&self) -> Option<Type> {
        match self {
            Type::Ptr(inner) => Some(*inner.clone()),
            _ => None,
        }
    }

    /// Creates a pointer to this type (e.g., `int` → `int*`).
    pub fn ptr_to(&self) -> Type {
        Type::Ptr(Box::new(self.clone()))
    }
}
