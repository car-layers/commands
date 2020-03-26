use capnpc::CompilerCommand;
use std::{error::Error, path::Path};

pub fn build_schemas(files: &[&str]) -> Result<(), impl Error> {
    let schema_dir = Path::new(file!())
        .parent()
        .unwrap()
        .join("../schema")
        .canonicalize()?;

    let mut cmd = CompilerCommand::new();
    cmd.src_prefix(schema_dir.as_path());
    for file in files {
        cmd.file(schema_dir.join(format!("{}.capnp", file)));
    }
    cmd.run()
}
