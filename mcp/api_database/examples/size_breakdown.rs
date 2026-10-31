//! One-shot size breakdown of api_database.bin
use api_database::{ApiDatabase, ApiItem, ApiItemEntry};
use indexmap::IndexMap;
use std::fs::File;
use std::io::BufReader;

fn mib(n: usize) -> f64 {
    n as f64 / 1024.0 / 1024.0
}

fn main() {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "/../data/api_database.bin");
    let f = File::open(path).expect("open api_database.bin");
    let db: ApiDatabase = bincode::deserialize_from(BufReader::new(f)).expect("deserialize");

    let n = db.items.len();
    println!("=== api_database.bin structure ===\n");
    println!("items: {n}");

    let mut by_type: IndexMap<&str, Vec<&ApiItemEntry>> = IndexMap::new();
    for item in db.items.values() {
        by_type.entry(item.item_type()).or_default().push(item);
    }

    println!("\ncounts + bincode size if that type alone were serialized as Vec:");
    let mut type_total = 0usize;
    for (ty, items) in &by_type {
        let bytes = bincode::serialize(items).unwrap().len();
        type_total += bytes;
        println!("  {ty:16} {:>6} items  {:>7.2} MiB", items.len(), mib(bytes));
    }
    println!("  {:16} {:>6}        {:>7.2} MiB (sum of per-type vecs)", "(sum)", "", mib(type_total));

    // Map overhead: keys + IndexMap structure
    let keys: Vec<&str> = db.items.keys().map(|s| s.as_str()).collect();
    let keys_bytes = bincode::serialize(&keys).unwrap().len();
    println!("\nmap keys only: {:.2} MiB", mib(keys_bytes));

    let items_only = ApiDatabase {
        items: db.items.clone(),
        embeddings: None,
    };
    let items_bytes = bincode::serialize(&items_only).unwrap().len();
    let emb = db.embeddings.as_ref().unwrap();
    let emb_bytes = bincode::serialize(emb).unwrap().len();
    let dim = emb[0].len();

    println!("\n=== top-level split ===");
    println!("  items (IndexMap):  {:.2} MiB  ({:.0}%)", mib(items_bytes), 100.0 * items_bytes as f64 / (items_bytes + emb_bytes) as f64);
    println!("  embeddings:        {:.2} MiB  ({:.0}%)", mib(emb_bytes), 100.0 * emb_bytes as f64 / (items_bytes + emb_bytes) as f64);
    println!("  file total:        {:.2} MiB", mib(std::fs::metadata(path).unwrap().len() as usize));
    println!("  embedding layout:  {n} × {dim} f32  (raw payload {:.2} MiB; bincode overhead {:.2} MiB)",
        mib(n * dim * 4), mib(emb_bytes - n * dim * 4));

    // Denormalization: methods live both under MO and as MethodItemData
    let mo_method_bytes: usize = by_type.get("managed_object").unwrap().iter().map(|i| {
        if let ApiItemEntry::ManagedObject(mo) = i {
            bincode::serialize(&mo.methods).unwrap().len()
        } else { 0 }
    }).sum();
    let method_entries_bytes = bincode::serialize(by_type.get("method").unwrap()).unwrap().len();
    println!("\n=== denormalization hotspots ===");
    println!("  methods nested under MOs:     {:.2} MiB", mib(mo_method_bytes));
    println!("  MethodItemData top-level:     {:.2} MiB (mostly duplicate of nested methods)", mib(method_entries_bytes));

    let field_under_struct: usize = by_type.get("structure").unwrap().iter().map(|i| {
        if let ApiItemEntry::Structure(s) = i {
            bincode::serialize(&s.fields).unwrap().len()
        } else { 0 }
    }).sum();
    let field_entries = bincode::serialize(by_type.get("field").unwrap()).unwrap().len();
    println!("  fields nested under structs:  {:.2} MiB", mib(field_under_struct));
    println!("  FieldItemData top-level:      {:.2} MiB (duplicate + paths + struct_description copies)", mib(field_entries));

    // paths on fields/structures
    let mut path_bytes = 0usize;
    let mut path_count = 0usize;
    for item in db.items.values() {
        match item {
            ApiItemEntry::Structure(s) => {
                path_count += s.paths.len();
                path_bytes += bincode::serialize(&s.paths).unwrap().len();
            }
            ApiItemEntry::Field(f) => {
                path_count += f.paths.len();
                path_bytes += bincode::serialize(&f.paths).unwrap().len();
            }
            _ => {}
        }
    }
    println!("  ApiTypePath payloads:         {:.2} MiB across {path_count} path entries", mib(path_bytes));

    // string gzip estimate via flate2? skip - use std::io::Write with no extra dep
    // rough: collect all description strings
    let mut all_desc = String::new();
    for item in db.items.values() {
        match item {
            ApiItemEntry::ManagedObject(mo) => {
                if let Some(d) = &mo.description { all_desc.push_str(d); }
            }
            ApiItemEntry::Method(m) => {
                if let Some(d) = &m.method.description { all_desc.push_str(d); }
            }
            ApiItemEntry::Structure(s) => {
                if let Some(d) = &s.description { all_desc.push_str(d); }
            }
            ApiItemEntry::Field(f) => {
                if let Some(d) = &f.field.description { all_desc.push_str(d); }
                if let Some(d) = &f.struct_description { all_desc.push_str(d); }
            }
            ApiItemEntry::Enumeration(e) => {
                if let Some(d) = &e.description { all_desc.push_str(d); }
            }
            ApiItemEntry::Trait(t) => {
                if let Some(d) = &t.description { all_desc.push_str(d); }
            }
            ApiItemEntry::Example(ex) => {
                all_desc.push_str(&ex.description);
                all_desc.push_str(&ex.source_code);
            }
        }
    }
    println!("\n=== text compression potential (descriptions+code concatenated, unique-ish) ===");
    println!("  raw concat: {:.2} MiB", mib(all_desc.len()));

    // lz4/zstd without deps: use `flate2` if available in workspace... use pure rust miniz via? 
    // Actually use zlib via libz? Simpler: shell gzip after writing
    std::fs::write("/tmp/api_desc_concat.txt", &all_desc).ok();

    let items_bin = bincode::serialize(&items_only).unwrap();
    std::fs::write("/tmp/api_items_only.bin", &items_bin).ok();
    let emb_bin = bincode::serialize(emb).unwrap();
    std::fs::write("/tmp/api_emb_only.bin", &emb_bin).ok();

    println!("\n(wrote /tmp/api_items_only.bin, /tmp/api_emb_only.bin, /tmp/api_desc_concat.txt for gzip)");

    println!("\n=== vector compression theoretical ===");
    println!("  f32:           {:.2} MiB", mib(n * dim * 4));
    println!("  f16:           {:.2} MiB", mib(n * dim * 2));
    println!("  int8:          {:.2} MiB", mib(n * dim));
    println!("  turbovec 4bit: {:.2} MiB codes (~8x)", mib(n * dim / 2));
    println!("  turbovec 2bit: {:.2} MiB codes (~16x)", mib(n * dim / 4));
}
