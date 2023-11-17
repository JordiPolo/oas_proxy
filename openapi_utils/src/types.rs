use openapiv3::*;

/// Extension methods for Type
pub trait TypeExt {
    /// true if this type is `Type::Boolean`
    fn is_bool(&self) -> bool;

    /// true if this type is `Type::Integer`
    fn is_integer(&self) -> bool;

    /// true if this type is `Type::Number`
    fn is_number(&self) -> bool;

    /// true if this type is `Type::String`
    fn is_string(&self) -> bool;
}

impl TypeExt for Type {
    fn is_bool(&self) -> bool {
        matches!(self, Type::Boolean {})
    }

    fn is_integer(&self) -> bool {
        matches!(self, Type::Integer(_))
    }

    fn is_number(&self) -> bool {
        matches!(self, Type::Number(_))
    }

    fn is_string(&self) -> bool {
        matches!(self, Type::String(_))
    }
}

/// Extends the `IntegerType` with convenience methods
pub trait IntegerTypeExt {
    /// Returns the minimum and maximum information as a tuple of i64
    /// If there is no minimum or maximum, the minimum i64 or maximum i64 values are used.
    fn min_max(&self) -> (i64, i64);
}

impl IntegerTypeExt for IntegerType {
    fn min_max(&self) -> (i64, i64) {
        let the_min = match self.minimum {
            Some(minimum) => {
                if self.exclusive_minimum {
                    minimum + 1
                } else {
                    minimum
                }
            }
            None => i64::min_value(),
        };

        let the_max = match self.maximum {
            Some(maximum) => {
                if self.exclusive_maximum {
                    maximum - 1
                } else {
                    maximum
                }
            }
            None => i64::max_value(),
        };
        (the_min, the_max)
    }
}


/// Extends the `NumberType` with convenience methods
pub trait NumberTypeExt {
    /// Returns the minimum and maximum information as a tuple of f64
    /// If there is no minimum or maximum, the minimum f64 or maximum f64 values are used.
    fn min_max(&self) -> (f64, f64);
}

impl NumberTypeExt for NumberType {
    fn min_max(&self) -> (f64, f64) {
        let the_min = match self.minimum {
            Some(minimum) => {
                if self.exclusive_minimum {
                    minimum + 1.0
                } else {
                    minimum
                }
            }
            None => core::f64::MIN,
        };

        let the_max = match self.maximum {
            Some(maximum) => {
                if self.exclusive_maximum {
                    maximum - 1.0
                } else {
                    maximum
                }
            }
            None => core::f64::MAX,
        };
        (the_min, the_max)
    }
}
