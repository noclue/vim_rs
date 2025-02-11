use std::collections::HashMap;

use indexmap::IndexMap;

use crate::vim_model::BoxType;
use crate::{printer::Printer, vim_model::Model};

use super::{to_type_name, TypeDefResolver};

use super::errors::{Result, Error};

const FULL_SCAN_THRESHOLD: usize = 5;


enum ItemRenderer {
    Object,
    Value,
}

pub struct DeserializationGenerator<'a> {
    vim_model: &'a Model,
    printer: &'a mut dyn Printer,
    any_value_types: IndexMap<String, &'a BoxType>,
    tdf: TypeDefResolver<'a>,
    deserialize_renderer: ItemRenderer,
}

impl DeserializationGenerator<'_> {
    pub fn new<'a>(vim_model: &'a Model, printer: &'a mut dyn Printer) -> DeserializationGenerator<'a> {
        let mut value_types: IndexMap<String, &BoxType> = IndexMap::new();
        for (name, box_type) in &vim_model.any_value_types {
            if name == "Any" {
                continue;
            }
            let key = box_type.discriminator_value.as_ref().unwrap_or(&name).clone();
            value_types.insert(key, &box_type);
        }
        DeserializationGenerator {
            vim_model,
            printer,
            any_value_types: value_types,
            tdf: TypeDefResolver::new(vim_model),
            deserialize_renderer: ItemRenderer::Object,
        }
    }

    pub fn generate_deserialization(&mut self) -> Result<()> {
        self.generate_vim_any_deserialization()?;
        self.generate_object_deserialization()?;
        self.generate_value_deserialization()?;
        Ok(())
    }
    pub fn generate_object_deserialization(&mut self) -> Result<()> {
        self.deserialize_renderer = ItemRenderer::Object;
        let names: &[&str] = &self.vim_model.structs.keys().map(|s| s.as_str()).filter(|v| *v != "Any" ).collect::<Vec<&str>>();

        let group_data = calculate_groupings(names);
        self.render_match_tree(group_data)?;

        Ok(())

    }

    pub fn generate_value_deserialization(&mut self) -> Result<()> {
        self.deserialize_renderer = ItemRenderer::Value;

        let names: &[&str] = &self.any_value_types.keys().map(|s| s.as_str()).collect::<Vec<&str>>();

        let group_data = calculate_groupings(names);
        self.render_match_tree(group_data)?;

        Ok(())
    }

    pub fn generate_vim_any_deserialization(&mut self) -> Result<()> {
        self.printer.println(r#"
fn to_u64(text: &str) -> Option<u64> {
    if text.len() != 8 {
        return None;
    }
    let bytes: &[u8; 8] = text.as_bytes().try_into().ok()?;
    Some(u64::from_be_bytes(*bytes))
}

fn to_u32(text: &str) -> Option<u32> {
    if text.len() != 4 {
        return None;
    }
    let bytes: &[u8; 4] = text.as_bytes().try_into().ok()?;
    Some(u32::from_be_bytes(*bytes))
}

impl<'de> de::Deserialize<'de> for VimAny {
    fn deserialize<D: de::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        deserializer.deserialize_map(VimAnyVisitor)
    }
}

struct VimAnyVisitor;

impl<'de> de::Visitor<'de> for VimAnyVisitor {
    type Value = VimAny;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("vim JSON object with _typeName field discrimnator")
    }

    fn visit_map<A: de::MapAccess<'de>>(self, mut map: A) -> Result<Self::Value, A::Error> {
        let mut type_name = None;
        let mut map_data: Vec<(String, &serde_json::value::RawValue)> = Vec::new();
        while let Some(key) = map.next_key::<String>()? {
            if key == "_typeName" {
                if type_name.is_none() {
                    let value: String = map.next_value()?;
                    type_name = Some(value);
                } // ignore subsequent _typeName fields
                if map_data.is_empty() {
                    break
                };
            } else {
                let value: &serde_json::value::RawValue = map.next_value()?;
                map_data.push((key, value));
            }
        }
        let Some(type_name) = type_name else {
                return Err(de::Error::missing_field("_typeName"));
        };
        if map_data.is_empty() { // Optimize the case when the first element is the discriminator
            // Attempt to deserialize object from type_name and map
            if let Some(dsfunc) = get_object_deserializer(&type_name) {
                let ds = de::value::MapAccessDeserializer::new(map);
                return dsfunc(ds).map_err(de::Error::custom);
            } else {
                let Some(dsfunc) = get_value_deserializer(&type_name) else {
                    return Err(de::Error::custom(format!("Unknown variant: {}", type_name)));
                };
                let Some(key) = map.next_key::<String>()? else {
                    return Err(de::Error::custom("Missing key"));
                };
                if key == "_value" {
                    let v: &serde_json::value::RawValue = map.next_value()?;
                    return dsfunc(v).map_err(de::Error::custom);
                }
                return Err(de::Error::custom(format!("Expected key '_value' and found {}", key)));
            }
        };

        // We have buffered all keys try to make sense of it
        // Process value elements
        if let Some(dsfunc) = get_object_deserializer(&type_name) {
            let map = de::value::MapDeserializer::new(map_data.into_iter());
            let ds = de::value::MapAccessDeserializer::new(map);
            return dsfunc(ds).map_err(de::Error::custom);
        }

        // Process value elements
        let Some(dsfunc) = get_value_deserializer(&type_name) else {
            return Err(de::Error::custom(format!("Unknown variant: {}", type_name)));
        };
        if map_data.len() == 1 && map_data[0].0 == "_value" {
            let v: &serde_json::value::RawValue = map_data
                .get(0)
                .ok_or_else(|| de::Error::missing_field("_value"))?
                .1;
            return dsfunc(v).map_err(de::Error::custom);
        }
        Err(de::Error::custom("Invalid format for boxed value element."))
    }
}"#)?;
        Ok(())
    }

    /// Renders a hierarchical match statement that dispatches to the individual names.
    fn render_match_tree(&mut self, group_data: Vec<GroupInfo>) -> Result<()> {
        match self.deserialize_renderer {
            ItemRenderer::Object => self.printer.println("fn get_object_deserializer<'de, A: de::MapAccess<'de>>(type_name: &str) -> Option<fn(de::value::MapAccessDeserializer<A>) -> Result<VimAny, A::Error>> {")?,
            ItemRenderer::Value => self.printer.println("fn get_value_deserializer(type_name: &str) -> Option<fn(&serde_json::value::RawValue) -> Result<VimAny, serde_json::Error>> {")?,
            
        }
        self.printer.indent();

        self.printer.println("match type_name.len() {")?;
        self.printer.indent();
        
        for group in &group_data {
            self.printer.println(&format!("{} => {{", group.length))?;
            self.printer.indent();
            if group.names.len() == 1 {
                self.process_simple_group(&group.names)?;
            } else {
                match self.deserialize_renderer {
                    ItemRenderer::Object => self.printer.println(&format!("get_object_deserializer_{}(type_name)",group.length))?,
                    ItemRenderer::Value => self.printer.println(&format!("get_value_deserializer_{}(type_name)",group.length))?,
                };
            }
            self.printer.dedent();
            self.printer.println("}")?;
        
        };
        self.printer.println("_ => None,")?;
        self.printer.dedent();
        self.printer.println("}")?;
        self.printer.dedent();
        self.printer.println("}")?;

        for group in &group_data {
            if group.names.len() == 1 {
                continue;
            };
            match self.deserialize_renderer {
                ItemRenderer::Object => self.printer.println(&format!("fn get_object_deserializer_{}<'de, A: de::MapAccess<'de>>(type_name: &str) -> Option<fn(de::value::MapAccessDeserializer<A>) -> Result<VimAny, A::Error>> {{", group.length))?,
                ItemRenderer::Value => self.printer.println(&format!("fn get_value_deserializer_{}(type_name: &str) -> Option<fn(&serde_json::value::RawValue) -> Result<VimAny, serde_json::Error>> {{", group.length))?,
                
            }
            self.printer.indent();
            if group.filter_len > 0 {
                self.process_complex_group(group)?;
            } else {
                self.process_simple_group(&group.names)?;
            }
            self.printer.dedent();
            self.printer.println("}")?;
        
        };

        Ok(())
    }

    fn deserialize_value_type(&mut self, name: &str) -> Result<()> {
        let Some(box_type) = self.any_value_types.get(name) else {
            return Err(Error::InternalError(format!("Cannot find value type record for {}", name)));
        };
        let enum_name = to_type_name(&box_type.name);
        let value_type = self.tdf.to_rust_field_type(&box_type.property_type)?;

        self.printer.println("Some(|raw| {")?;
        self.printer.indent();
        self.printer.println(&format!("let value: {} = serde_json::from_str(raw.get())?;", value_type))?;
        self.printer.println(&format!("Ok(VimAny::Value(ValueElements::{}(value)))", enum_name))?;
        self.printer.dedent();
        self.printer.println("})")?;
        Ok(())
    }

    fn deserialize_object_type(&mut self, name: &str) -> Result<()> {
        self.printer.println("Some(|ds| {")?;
        self.printer.indent();
        self.printer.println(&format!("let obj: {} = de::Deserialize::deserialize(ds)?;", to_type_name(name)))?;
        self.printer.println("Ok(VimAny::Object(Box::new(obj)))")?;
        self.printer.dedent();
        self.printer.println("})")?;
        Ok(())
    }

    fn process_simple_group(&mut self, names: &[String]) -> Result<()> {
        if names.is_empty() {
            return Err(Error::InternalError("No names provided to process_simple_group".into()));
        }
        if names.len() == 1 {
            self.printer.println(&format!("if type_name == \"{}\" {{", names[0]))?;
            self.printer.indent();
            match self.deserialize_renderer {
                ItemRenderer::Object => self.deserialize_object_type(&names[0])?,
                ItemRenderer::Value => self.deserialize_value_type(&names[0])?,
            }
            self.printer.dedent();
            self.printer.println("} else { None }")?;
            return Ok(());
        } 
    
        self.printer.println("match type_name {")?;
        self.printer.indent();
        for name in names {
            self.printer.println(&format!("\"{}\" => {{", name))?;
            self.printer.indent();
            match self.deserialize_renderer {
                ItemRenderer::Object => self.deserialize_object_type(name)?,
                ItemRenderer::Value => self.deserialize_value_type(name)?,
            }
            self.printer.dedent();
            self.printer.println("}")?;
        }
        self.printer.println("_ => None")?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }
    
    fn process_complex_group(&mut self, group: &GroupInfo) -> Result<()> {
        if group.names.is_empty() {
            return Err(Error::InternalError("No names provided to process_complex_group".into()));
        }
        self.printer.println(&format!("let s = &type_name[{}..{}];", group.position, group.position + group.filter_len))?;
        if group.filter_len == 4 {
            self.printer.println(r#"let Some(type_ord) = to_u32(s) else {"#)?;
        } else if group.filter_len == 8 {
            self.printer.println(r#"let Some(type_ord) = to_u64(s) else {"#)?;
        } else {
            return Err(Error::InternalError(format!("Unsupported filter length: {}", group.filter_len)));
        }
        self.printer.indent();
        self.printer.println(r#"return None;"#)?;
        self.printer.dedent();
        self.printer.println("};")?;
        let mut names = group.names.clone();
        names.sort_by(|a, b| a[group.position..].cmp(&b[group.position..]));
        self.printer.println("match type_ord {")?;
        self.printer.indent();
        let mut subgroup= Vec::new();
        let mut previous = "".to_string();
        for name in &names {
            let value = name[group.position..(group.position + group.filter_len)].to_string();
            if previous.is_empty() || previous != value {
                // A subgroup is starting here and if not first it is ending too
                self.render_subgroup(&mut subgroup, &previous)?;
                previous = value;
            }
            subgroup.push(name.clone());
        }
        self.render_subgroup(&mut subgroup, &previous)?;
        self.printer.println("_ => None")?;
        self.printer.dedent();
        self.printer.println("}")?;
        Ok(())
    }
    
    fn render_subgroup(&mut self, subgroup: &mut Vec<String>, pattern: &String) -> Result<()> {
        Ok(if subgroup.len() >= 1 {
            let Some(numeric) = to_numeric_value(pattern) else {
                return Err(Error::InternalError(format!("Cannot convert pattern into numeric: {}", pattern)));
            };
            self.printer.println(&format!("{} => {{ // {}", numeric, pattern))?;
            self.printer.indent();    
            self.process_simple_group(subgroup.as_slice())?;
            self.printer.dedent();
            self.printer.println("},")?;
            subgroup.clear();
        })
    }



}

/// Groups names by length and for each group find the best way to dispatch. 
/// If a group is less then FULL_SCAN_THRESHOLD we use simple match statement by the full name.
/// If a group is larger we try to find index of a 4 or 8 letter long substring that will allows to
/// split the group into subgroups with minimal number of elements.
fn calculate_groupings(names: &[&str]) -> Vec<GroupInfo> {
    let groups = split_by_length(names);
    
    let mut sorted_groups: Vec<_> = groups.into_iter().collect();
    sorted_groups.sort_by_key(|(len, _)| *len);

    let mut group_data = Vec::new();
    const DEFAULT_PARAMS: Params = Params{ position: 0, len: 0 };

    for (length, names) in sorted_groups {
        let params = optimize_group(&names);
        //println!("{} (cnt: {})\t->\t{:?}", length, names.len(), params);
        group_data.push(GroupInfo{
            length: length,
            names: names,
            position: params.as_ref().unwrap_or(&DEFAULT_PARAMS).position,
            filter_len: params.as_ref().unwrap_or(&DEFAULT_PARAMS).len,
        }); 
    }
    group_data
}

#[derive(Debug)]
struct Params {
    position: usize,
    len: usize,
}

struct EvalResult {
    position: usize,
    max_subgroup_len: usize,
}

struct GroupInfo {
    length: usize,
    names: Vec<String>,
    position: usize,
    filter_len: usize,
}

fn split_by_length(names: &[&str]) -> HashMap<usize, Vec<String>> {
    let mut groups: HashMap<usize, Vec<String>> = HashMap::new();

    // Group names by length
    for &name in names {
        let len = name.len();
        groups.entry(len).or_default().push(name.to_string());
    }
    groups
}

// Removes characters up to position and sorts the remaining characters
fn cut_and_sort_names(position: usize, names: &[String]) -> Vec<String> {
    let mut filtered: Vec<String> = names.iter().map(|name| name[position..].to_string()).collect();
    filtered.sort();
    filtered
}

fn eval_filter_len(filter_len: usize, names: &[String]) -> EvalResult {
    let empty = String::new();
    let pattern_len = names[0].len();
    let mut optimal_position = 0;
    let mut minimal_max_subgroup_len = usize::MAX;

    let mut position = 0;
    while position <= (pattern_len-filter_len) {
        let mut prior = &empty;
        let mut subgroup_len = 1;
        let mut max_subgroup_len = 1;
        let cut_names = cut_and_sort_names(position, names);
        for name in &cut_names {
            if prior.is_empty() || name[..filter_len] != prior[..filter_len] {
                prior = name;
                subgroup_len = 1;
            } else {
                subgroup_len += 1;
                max_subgroup_len = std::cmp::max(max_subgroup_len, subgroup_len);
            }
        }
        if max_subgroup_len < minimal_max_subgroup_len {
            minimal_max_subgroup_len = max_subgroup_len;
            optimal_position = position;
        }
        if max_subgroup_len == 1 { break; };
        position += 1;
    }
    EvalResult { position: optimal_position, max_subgroup_len: minimal_max_subgroup_len }
}

fn optimize_group(names: &[String]) -> Option<Params> {
    if names.len()  < FULL_SCAN_THRESHOLD {
        return None;
    }

    let mut best_param= None;
    let mut min_subgroup_len = names.len();

    for filter_len in (vec![4,8]).into_iter() {
        let res = eval_filter_len(filter_len, names);
        if res.max_subgroup_len < min_subgroup_len {
            best_param = Some(Params { position: res.position, len: filter_len });
            min_subgroup_len = res.max_subgroup_len;
        }
        if min_subgroup_len == 1 {
            break;
        }
    };
    best_param
}


fn to_u64(text: &str) -> Option<u64> {
    if text.len() != 8 {
        return None;
    }
    let bytes: &[u8; 8] = text.as_bytes().try_into().ok()?;
    Some(u64::from_be_bytes(*bytes))
}

fn to_u32(text: &str) -> Option<u32> {
    if text.len() != 4 {
        return None;
    }
    let bytes: &[u8; 4] = text.as_bytes().try_into().ok()?;
    Some(u32::from_be_bytes(*bytes))
}

fn to_numeric_value(text: &str) -> Option<String> {
    if text.len() == 8 {
        return to_u64(text).map(|v| format!("{:#x}", v));
    } else if text.len() == 4 {
        return to_u32(text).map(|v| format!("{:#x}", v));
    }
    None
}