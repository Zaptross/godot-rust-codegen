use std::{fs::File, io::Write, path::Path};

const MOD_FILE: &str = "mod.rs";

pub fn write_mod_file(output_dir: &str, mods: Vec<String>) {
    let mod_file_path = Path::new(output_dir).join(MOD_FILE);
    let mut mod_file = File::create(mod_file_path).unwrap();

    for m in mods {
        if m.is_empty() {
            continue;
        }

        mod_file
            .write(format!("pub mod {};\n", m).as_bytes())
            .unwrap();
    }
}
