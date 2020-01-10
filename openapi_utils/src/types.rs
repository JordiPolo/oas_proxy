use core::ops::Range;
use openapiv3::*;

/// Extension methods for Type
pub trait TypeExt {
    /// true if this type is Type::Boolean
    fn is_bool(&self) -> bool;

    /// true if this type is Type::Integer
    fn is_integer(&self) -> bool;

    /// true if this type is Type::Number
    fn is_number(&self) -> bool;

    /// true if this type is Type::String
    fn is_string(&self) -> bool;
}

impl TypeExt for Type {
    fn is_bool(&self) -> bool {
        match self {
            Type::Boolean {} => true,
            _ => false,
        }
    }

    fn is_integer(&self) -> bool {
        match self {
            Type::Integer(_) => true,
            _ => false,
        }
    }

    fn is_number(&self) -> bool {
        match self {
            Type::Number(_) => true,
            _ => false,
        }
    }

    fn is_string(&self) -> bool {
        match self {
            Type::String(_) => true,
            _ => false,
        }
    }
}

/// Extends the IntergerType with convenience methods
pub trait IntegerTypeExt {
    /// Returns the minimum and maximum information as a Range of i64
    /// If there is no minimum or maximum, the minimum i64 or maximum i64 values are used.
    fn limits(&self) -> Range<i64>;
}

impl IntegerTypeExt for IntegerType {
    fn limits(&self) -> Range<i64> {
        let the_min = match self.minimum {
            Some(minimum) => {
                if self.exclusive_minimum {
                    minimum + 1
                } else {
                    minimum
                }
            }
            None => core::i64::MIN + 1,
        };

        let the_max = match self.maximum {
            Some(maximum) => {
                if self.exclusive_maximum {
                    maximum - 1
                } else {
                    maximum
                }
            }
            None => core::i64::MAX - 1,
        };
        core::ops::Range {
            start: the_min,
            end: the_max,
        }
    }
}
