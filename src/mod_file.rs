use std::{fs, io::Write};

const MOD_FILE: &str = "mod.rs";

pub fn write_mod_file(output_dir: &str, mods: Vec<String>) {
    let mod_file_path = if (!output_dir.ends_with('/')) && (!output_dir.ends_with('\\')) {
        format!("{}/{}", output_dir, MOD_FILE)
    } else {
        format!("{}{}", output_dir, MOD_FILE)
    };

    let mut mod_file = fs::File::create(mod_file_path).unwrap();

    for m in mods {
        if m.is_empty() {
            continue;
        }

        mod_file
            .write(format!("pub mod {};\n", m).as_bytes())
            .unwrap();
    }
}
