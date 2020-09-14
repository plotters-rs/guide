use pulldown_cmark::{Parser, Event, Tag, CodeBlockKind};
use std::fs::File;
use std::io::{Read, Write};
use std::path::{Path, Component};
fn make_prefix(path: &Path) -> String {
    let mut prefix = String::new();
    for name in path.components().filter_map(|c| match c {
        Component::Normal(name) => Some(name),
        _ => None,
    }).skip(1).filter_map(|name| name.to_str()) {
        let name = AsRef::<Path>::as_ref(name).file_stem().unwrap().to_str().unwrap();
        prefix.push_str(name);
        prefix.push_str("_");
    }
    prefix
}
fn extract_code<P:AsRef<Path>>(path:P) -> Result<usize, Box<dyn std::error::Error>> {
    let mut prefix = "src/".to_string();

    prefix.push_str(&make_prefix(path.as_ref()));
    
    let mut content = String::new();
    File::open(path)?.read_to_string(&mut content)?;

    let parser = Parser::new(&content);

    let mut in_rust_code = false;

    let mut counter = 1;

    for event in parser {
        match event {
            Event::Start(Tag::CodeBlock(CodeBlockKind::Fenced(ref lang))) if lang.as_ref() == "rust"  => in_rust_code = true,
            Event::Code(ref code) | Event::Text(ref code) if in_rust_code => {
                let filename = format!("{}{}.rs", prefix, counter);
                let mut code = code.clone().into_string();
                code.push_str("\npub fn entry_point() { main(); } \n");
                File::create(&filename)?.write_all(code.as_bytes())?;
                counter += 1;
                in_rust_code = false;
            },
            _ => in_rust_code = false,
        }
    }

    Ok(counter)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut main_file = File::create("src/lib.rs")?;

    for entry in walkdir::WalkDir::new("markdown").into_iter().filter_map(Result::ok).filter(|x| x.file_type().is_file()) {
        println!("cargo:rerun-if-changed={:?}", entry.path());
        if let Some(ext) = entry.path().extension() {
            if ext == "md" {
                let prefix = make_prefix(entry.path());
                for i in 1..extract_code(entry.path())? {
                    main_file.write_all(format!("pub mod {}{};\n", prefix, i).as_bytes())?;
                    main_file.write_all(format!("#[test]\n fn {}{}_entry_point(){{ {}{}::entry_point(); }}\n\n", prefix, i, prefix, i).as_bytes())?;
                }
            }
        }
    }


    Ok(())
}
