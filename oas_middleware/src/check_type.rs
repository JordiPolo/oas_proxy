use crate::error::{maximum_error, minimum_error, type_error};

use crate::error::E;
use chrono::{DateTime, FixedOffset};
use openapiv3::*;
use regex::Regex;
use uuid::Uuid;

use crate::request::Attribute;

use openapi_utils::IntegerTypeExt;

pub fn check_type(the_type: &Type, request_param_data: &Attribute) -> Result<(), E> {
    match the_type {
        Type::String(StringType { format, .. }) => match format {
            VariantOrUnknownOrEmpty::Item(string_format) => match string_format {
                StringFormat::Date => check_date(&request_param_data),
                StringFormat::DateTime => check_datetime(&request_param_data),
                StringFormat::Byte => check_base64(&request_param_data),
                _ => Err(E::TypeNotsupported("String format".to_string())), //Ok(()),
            },
            VariantOrUnknownOrEmpty::Unknown(string) => {
                if string == "uuid" {
                    check_uuid(&request_param_data)
                } else {
                    check_plain_string(&request_param_data)
                }
            }
            VariantOrUnknownOrEmpty::Empty => check_plain_string(&request_param_data),
        },
        Type::Integer(integer_type) => check_integer(request_param_data, integer_type),
        Type::Number(number_type) => check_number(request_param_data, number_type),
        Type::Boolean {} => check_boolean(request_param_data),
        Type::Object(object_type) => check_object(request_param_data, object_type),
        Type::Array(_array_type) => Err(E::TypeNotsupported("Array".to_string())),
    }
}

fn check_object(attribute: &Attribute, number_type: &ObjectType) -> Result<(), E> {
    Err(E::TypeNotsupported("Object".to_string()))
}

fn read_integer(attribute: &Attribute, integer_type: &IntegerType) -> Result<i64, E> {
    match &integer_type.format {
        VariantOrUnknownOrEmpty::Item(IntegerFormat::Int32) => match attribute.value.parse::<i32>()
        {
            Ok(number) => Ok(number.into()),
            Err(_) => Err(type_error("integer int32", &attribute)),
        },
        VariantOrUnknownOrEmpty::Item(IntegerFormat::Int64) => attribute
            .value
            .parse::<i64>()
            .map_err(|_| type_error("integer int64", &attribute)),
        VariantOrUnknownOrEmpty::Unknown(_format_name) => attribute
            .value
            .parse::<i64>()
            .map_err(|_| type_error("integer unknown format", &attribute)),
        VariantOrUnknownOrEmpty::Empty => attribute
            .value
            .parse::<i64>()
            .map_err(|_| type_error("integer", &attribute)),
    }
}

fn read_float(attribute: &Attribute, integer_type: &NumberType) -> Result<f64, E> {
    match &integer_type.format {
        VariantOrUnknownOrEmpty::Item(NumberFormat::Float) => {
            match attribute.value.parse::<f32>() {
                Ok(number) => Ok(number.into()),
                Err(_) => Err(type_error("float", &attribute)),
            }
        }
        VariantOrUnknownOrEmpty::Item(NumberFormat::Double) => attribute
            .value
            .parse::<f64>()
            .map_err(|_| type_error("double", &attribute)),
        VariantOrUnknownOrEmpty::Unknown(_format_name) => attribute
            .value
            .parse::<f64>()
            .map_err(|_| type_error("float unknown format", &attribute)),
        VariantOrUnknownOrEmpty::Empty => attribute
            .value
            .parse::<f64>()
            .map_err(|_| type_error("float", &attribute)),
    }
}


fn check_integer(attribute: &Attribute, integer_type: &IntegerType) -> Result<(), E> {
    let number = read_integer(attribute, &integer_type)?;
    let min_max = integer_type.limits();

    if !integer_type.enumeration.is_empty() {
        integer_type.enumeration.contains(&number);
        // TODO: Check this.
    }

    if number < min_max.start {
        Err(minimum_error(&min_max.start.to_string(), attribute))
    } else if number > min_max.end {
        Err(maximum_error(&min_max.end.to_string(), attribute))
    } else {
        Ok(())
    }
}

fn check_number(attribute: &Attribute, number_type: &NumberType) -> Result<(), E> {
    let _number = read_float(attribute, &number_type)?;
    //e check_integer_limits(number, attribute, integer_type)?;

    Ok(())
}

fn check_boolean(attribute: &Attribute) -> Result<(), E> {
    //attribute.value.parse::<bool>().map_err(|_| Err(type_error("boolean", &attribute)))
    match attribute.value.parse::<bool>() {
        Ok(_) => Ok(()),
        Err(_) => Err(type_error("boolean", &attribute)),
    }
}

fn check_uuid(attribute: &Attribute) -> Result<(), E> {
    match Uuid::parse_str(&attribute.value) {
        Ok(_) => Ok(()),
        Err(_) => Err(type_error("UUID", &attribute)),
    }
}

// TODO: Can't tell if time was missing
fn check_date(attribute: &Attribute) -> Result<(), E> {
    match DateTime::<FixedOffset>::parse_from_rfc3339(&attribute.value) {
        Ok(_) => Ok(()),
        Err(_) => Err(type_error("Date", &attribute)),
    }
}

fn check_datetime(attribute: &Attribute) -> Result<(), E> {
    match DateTime::<FixedOffset>::parse_from_rfc3339(&attribute.value) {
        Ok(_) => Ok(()),
        Err(_) => Err(type_error("Datetime", &attribute)),
    }
}

fn check_base64(attribute: &Attribute) -> Result<(), E> {
    let string = "^([A-Za-z0-9+/]{4})*([A-Za-z0-9+/]{3}=|[A-Za-z0-9+/]{2}==)?$";
    let regex = Regex::new(&string).expect("Could not create base64 regex");

    if regex.is_match(&attribute.value) {
        Ok(())
    } else {
        Err(type_error("Base64 string", &attribute))
    }
}

fn reverse_result(a: Result<(), E>, attribute: &Attribute) -> Result<(), E> {
    match a {
        Ok(_) => Err(type_error("string without format", &attribute)),
        Err(_) => Ok(()),
    }
}

// TODO Check the format is not numeral or integer
fn check_plain_string(attribute: &Attribute) -> Result<(), E> {
    reverse_result(check_boolean(attribute), &attribute)
        .and(reverse_result(check_uuid(attribute), &attribute))
        .and(reverse_result(check_date(attribute), &attribute))
        .and(reverse_result(check_datetime(attribute), &attribute))
}
