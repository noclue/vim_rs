//! Typed miniserde visitors that build `json::Value` using `ApiFieldType` metadata.

#![cfg(feature = "xml")]

use std::collections::BTreeMap;
use std::mem;

use miniserde::de::{Deserialize, Map, Seq, Visitor};
use miniserde::json::{Array, Number, Object, Value};

use super::api_field_registry::lookup_api_field;
use super::api_field_types::ApiFieldType;
use super::struct_enum::StructType;

/// Visitor that writes a [`Value`] according to an [`ApiFieldType`].
pub struct ApiTypedValueVisitor {
    field_type: ApiFieldType,
    out: Option<Value>,
}

impl ApiTypedValueVisitor {
    pub fn new() -> Self {
        Self {
            field_type: ApiFieldType::Str,
            out: None,
        }
    }

    pub fn reset(&mut self, ft: ApiFieldType) {
        self.field_type = ft;
        self.out = None;
    }

    pub fn take_value(&mut self) -> Option<Value> {
        self.out.take()
    }

    fn reject(&self) -> miniserde::Result<()> {
        Err(miniserde::Error)
    }

    fn parse_bool_str(s: &str) -> Option<bool> {
        match s.trim().to_ascii_lowercase().as_str() {
            "true" | "1" => Some(true),
            "false" | "0" => Some(false),
            _ => None,
        }
    }

    fn parse_int_string(s: &str) -> Option<i64> {
        s.trim().parse::<i64>().ok()
    }

    fn parse_float_string(s: &str) -> Option<f64> {
        s.trim().parse::<f64>().ok()
    }
}

impl Default for ApiTypedValueVisitor {
    fn default() -> Self {
        Self::new()
    }
}

impl Visitor for ApiTypedValueVisitor {
    fn null(&mut self) -> miniserde::Result<()> {
        self.out = Some(Value::Null);
        Ok(())
    }

    fn boolean(&mut self, b: bool) -> miniserde::Result<()> {
        match self.field_type {
            ApiFieldType::Bool => {
                self.out = Some(Value::Bool(b));
                Ok(())
            }
            ApiFieldType::Any => Deserialize::begin(&mut self.out).boolean(b),
            _ => self.reject(),
        }
    }

    fn string(&mut self, s: &str) -> miniserde::Result<()> {
        match self.field_type {
            ApiFieldType::Bool => {
                let b = Self::parse_bool_str(s).ok_or(miniserde::Error)?;
                self.out = Some(Value::Bool(b));
                Ok(())
            }
            ApiFieldType::I8 => {
                let n = Self::parse_int_string(s).ok_or(miniserde::Error)?;
                if n < i8::MIN as i64 || n > i8::MAX as i64 {
                    return Err(miniserde::Error);
                }
                self.out = Some(Value::Number(Number::I64(n)));
                Ok(())
            }
            ApiFieldType::I16 => {
                let n = Self::parse_int_string(s).ok_or(miniserde::Error)?;
                if n < i16::MIN as i64 || n > i16::MAX as i64 {
                    return Err(miniserde::Error);
                }
                self.out = Some(Value::Number(Number::I64(n)));
                Ok(())
            }
            ApiFieldType::I32 => {
                let n = Self::parse_int_string(s).ok_or(miniserde::Error)?;
                if n < i32::MIN as i64 || n > i32::MAX as i64 {
                    return Err(miniserde::Error);
                }
                self.out = Some(Value::Number(Number::I64(n)));
                Ok(())
            }
            ApiFieldType::I64 => {
                let n = Self::parse_int_string(s).ok_or(miniserde::Error)?;
                self.out = Some(Value::Number(Number::I64(n)));
                Ok(())
            }
            ApiFieldType::F32 => {
                let n = Self::parse_float_string(s).ok_or(miniserde::Error)?;
                let f = n as f32;
                if !f.is_finite() {
                    return Err(miniserde::Error);
                }
                self.out = Some(Value::Number(Number::F64(f as f64)));
                Ok(())
            }
            ApiFieldType::F64 => {
                let n = Self::parse_float_string(s).ok_or(miniserde::Error)?;
                self.out = Some(Value::Number(Number::F64(n)));
                Ok(())
            }
            ApiFieldType::Str | ApiFieldType::Binary => {
                self.out = Some(Value::String(s.to_owned()));
                Ok(())
            }
            ApiFieldType::Array(_) => self.reject(),
            ApiFieldType::Object(_) | ApiFieldType::Any => self.reject(),
        }
    }

    fn negative(&mut self, n: i64) -> miniserde::Result<()> {
        match self.field_type {
            ApiFieldType::I8 => {
                if n < i8::MIN as i64 || n > i8::MAX as i64 {
                    return Err(miniserde::Error);
                }
                self.out = Some(Value::Number(Number::I64(n)));
                Ok(())
            }
            ApiFieldType::I16 => {
                if n < i16::MIN as i64 || n > i16::MAX as i64 {
                    return Err(miniserde::Error);
                }
                self.out = Some(Value::Number(Number::I64(n)));
                Ok(())
            }
            ApiFieldType::I32 => {
                if n < i32::MIN as i64 || n > i32::MAX as i64 {
                    return Err(miniserde::Error);
                }
                self.out = Some(Value::Number(Number::I64(n)));
                Ok(())
            }
            ApiFieldType::I64 => {
                self.out = Some(Value::Number(Number::I64(n)));
                Ok(())
            }
            ApiFieldType::F32 | ApiFieldType::F64 => {
                self.out = Some(Value::Number(Number::F64(n as f64)));
                Ok(())
            }
            ApiFieldType::Any => Deserialize::begin(&mut self.out).negative(n),
            _ => self.reject(),
        }
    }

    fn nonnegative(&mut self, n: u64) -> miniserde::Result<()> {
        match self.field_type {
            ApiFieldType::I8 => {
                if n > i8::MAX as u64 {
                    return Err(miniserde::Error);
                }
                self.out = Some(Value::Number(Number::I64(n as i64)));
                Ok(())
            }
            ApiFieldType::I16 => {
                if n > i16::MAX as u64 {
                    return Err(miniserde::Error);
                }
                self.out = Some(Value::Number(Number::I64(n as i64)));
                Ok(())
            }
            ApiFieldType::I32 => {
                if n > i32::MAX as u64 {
                    return Err(miniserde::Error);
                }
                self.out = Some(Value::Number(Number::I64(n as i64)));
                Ok(())
            }
            ApiFieldType::I64 => {
                if n <= i64::MAX as u64 {
                    self.out = Some(Value::Number(Number::I64(n as i64)));
                } else {
                    self.out = Some(Value::Number(Number::U64(n)));
                }
                Ok(())
            }
            ApiFieldType::F32 | ApiFieldType::F64 => {
                self.out = Some(Value::Number(Number::F64(n as f64)));
                Ok(())
            }
            ApiFieldType::Any => Deserialize::begin(&mut self.out).nonnegative(n),
            _ => self.reject(),
        }
    }

    fn float(&mut self, n: f64) -> miniserde::Result<()> {
        match self.field_type {
            ApiFieldType::F32 | ApiFieldType::F64 => {
                self.out = Some(Value::Number(Number::F64(n)));
                Ok(())
            }
            ApiFieldType::Any => Deserialize::begin(&mut self.out).float(n),
            _ => self.reject(),
        }
    }

    fn seq(&mut self) -> miniserde::Result<Box<dyn Seq + '_>> {
        match self.field_type {
            ApiFieldType::Array(inner) => Ok(Box::new(ApiTypedSeqBuilder {
                inner_type: *inner,
                items: Vec::new(),
                element_visitor: ApiTypedValueVisitor::new(),
                out: &mut self.out,
            })),
            ApiFieldType::Any => Deserialize::begin(&mut self.out).seq(),
            _ => Err(miniserde::Error),
        }
    }

    fn map(&mut self) -> miniserde::Result<Box<dyn Map + '_>> {
        match self.field_type {
            ApiFieldType::Object(st) => Ok(Box::new(ApiTypedMapBuilder {
                declared_type: st,
                effective_type: None,
                fields: BTreeMap::new(),
                current_key: None,
                field_visitor: ApiTypedValueVisitor::new(),
                out: &mut self.out,
            })),
            ApiFieldType::Any => Deserialize::begin(&mut self.out).map(),
            _ => Err(miniserde::Error),
        }
    }
}

struct ApiTypedSeqBuilder<'a> {
    inner_type: ApiFieldType,
    items: Vec<Value>,
    element_visitor: ApiTypedValueVisitor,
    out: &'a mut Option<Value>,
}

impl Seq for ApiTypedSeqBuilder<'_> {
    fn element(&mut self) -> miniserde::Result<&mut dyn Visitor> {
        if let Some(v) = self.element_visitor.take_value() {
            self.items.push(v);
        }
        self.element_visitor.reset(self.inner_type);
        Ok(&mut self.element_visitor)
    }

    fn finish(&mut self) -> miniserde::Result<()> {
        if let Some(v) = self.element_visitor.take_value() {
            self.items.push(v);
        }
        let mut arr = Array::new();
        for v in mem::take(&mut self.items) {
            arr.push(v);
        }
        *self.out = Some(Value::Array(arr));
        Ok(())
    }
}

struct ApiTypedMapBuilder<'a> {
    declared_type: StructType,
    effective_type: Option<StructType>,
    fields: BTreeMap<String, Value>,
    current_key: Option<String>,
    field_visitor: ApiTypedValueVisitor,
    out: &'a mut Option<Value>,
}

impl Map for ApiTypedMapBuilder<'_> {
    fn key(&mut self, key: &str) -> miniserde::Result<&mut dyn Visitor> {
        self.shift();
        self.current_key = Some(key.to_owned());

        if key == "_typeName" {
            self.field_visitor.reset(ApiFieldType::Str);
            return Ok(&mut self.field_visitor);
        }

        let lookup_type = self.effective_type.unwrap_or(self.declared_type);

        if let Some(ft) = lookup_api_field(lookup_type, key) {
            self.field_visitor.reset(ft);
        } else {
            self.field_visitor.reset(ApiFieldType::Str);
        }
        Ok(&mut self.field_visitor)
    }

    fn finish(&mut self) -> miniserde::Result<()> {
        self.shift();
        *self.out = Some(Value::Object(Object::from_iter(mem::take(&mut self.fields))));
        Ok(())
    }
}

impl ApiTypedMapBuilder<'_> {
    fn shift(&mut self) {
        if let (Some(k), Some(v)) = (self.current_key.take(), self.field_visitor.take_value()) {
            if k == "_typeName" {
                if let Value::String(ref s) = v {
                    self.effective_type = StructType::from_str(s);
                }
            }
            self.fields.insert(k, v);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn xml_style_i32_via_string() {
        let mut v = ApiTypedValueVisitor::new();
        v.reset(ApiFieldType::I32);
        v.string("403").unwrap();
        match v.take_value() {
            Some(Value::Number(Number::I64(n))) => assert_eq!(n, 403),
            o => panic!("expected I64(403), got {:?}", o),
        }
    }

    #[test]
    fn bool_via_string() {
        let mut v = ApiTypedValueVisitor::new();
        v.reset(ApiFieldType::Bool);
        v.string("true").unwrap();
        match v.take_value() {
            Some(Value::Bool(b)) => assert!(b),
            o => panic!("expected Bool(true), got {:?}", o),
        }
    }

    #[test]
    fn array_accepts_seq_not_string() {
        static INNER: ApiFieldType = ApiFieldType::I32;
        let mut v = ApiTypedValueVisitor::new();
        v.reset(ApiFieldType::Array(&INNER));
        assert!(v.string("1").is_err());
    }
}
