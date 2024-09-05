use std::{
    ffi::OsStr,
    fs::{self, File},
    io::Write,
};

// generated by `sqlx migrate build-script`
fn main() {
    // trigger recompilation when a new migration is added
    println!("cargo:rerun-if-changed=migrations");

    gen_binding_index();
}

// From https://github.com/Aleph-Alpha/ts-rs/issues/133#issuecomment-1399589933
fn gen_binding_index() {
    println!("generating bindings/index.ts");

    let exports: Vec<_> = fs::read_dir("./bindings")
        .unwrap()
        .filter_map(Result::ok)
        .filter_map(|p| {
            p.path()
                .file_stem()
                .map(OsStr::to_str)
                .flatten()
                .map(str::to_owned)
        })
        .filter(|f| f != "index")
        .map(|f| format!("export * from \"./{}\"", f))
        .collect();

    let mut file = File::create("./bindings/index.ts").unwrap();
    file.write_all(exports.join("\n").as_bytes()).unwrap();
}
