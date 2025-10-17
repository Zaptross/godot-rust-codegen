use std::fs;

pub fn pascal_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}
#[test]
fn test_pascal_to_snake_case() {
    assert_eq!(pascal_to_snake_case("Fire"), "fire");
    assert_eq!(pascal_to_snake_case("JumpAction"), "jump_action");
    assert_eq!(pascal_to_snake_case("A"), "a");
    assert_eq!(pascal_to_snake_case(""), "");
}

/// Converts a string to UpperCamelCase.
///
/// e.g. "example_string" -> "ExampleString"
pub fn to_upper_camel_case(s: &str) -> String {
    let mut result = String::new();
    let mut capitalize = true;
    for c in s.chars() {
        if c == '_' {
            capitalize = true;
        } else if capitalize {
            result.push(c.to_ascii_uppercase());
            capitalize = false;
        } else {
            result.push(c);
        }
    }
    result
}

#[test]
fn test_to_upper_camel_case() {
    assert_eq!(to_upper_camel_case("example_string"), "ExampleString");
    assert_eq!(to_upper_camel_case("another_example"), "AnotherExample");
    assert_eq!(to_upper_camel_case("single"), "Single");
    assert_eq!(to_upper_camel_case(""), "");
}

/// Converts a string from UpperCamelCase to snake_case.
///
/// e.g. "ExampleString" -> "example_string"
pub fn upper_camel_to_snake_case(s: &str) -> String {
    let mut result = String::new();
    for (i, c) in s.chars().enumerate() {
        if c.is_uppercase() && i > 0 {
            result.push('_');
        }
        result.push(c.to_ascii_lowercase());
    }
    result
}

#[test]
fn test_upper_camel_to_snake_case() {
    assert_eq!(upper_camel_to_snake_case("ExampleString"), "example_string");
    assert_eq!(
        upper_camel_to_snake_case("AnotherExample"),
        "another_example"
    );
    assert_eq!(upper_camel_to_snake_case("Single"), "single");
    assert_eq!(upper_camel_to_snake_case(""), "");
}

pub fn to_resource_path(path: &str, resource_path: &str) -> String {
    path.replace(resource_path, "res:/")
}
#[test]
fn test_to_resource_path() {
    let resource_path = "C:/Projects/MyGame/gd";
    let path = "C:/Projects/MyGame/gd/scenes/Main.tscn";
    let expected = "res://scenes/Main.tscn";
    assert_eq!(to_resource_path(path, resource_path), expected);
}

pub fn make_path_if_not_exists(path: &str) {
    let path_obj = std::path::Path::new(path);
    if !path_obj.exists() {
        if let Some(parent) = path_obj.parent() {
            if !parent.exists() {
                fs::create_dir_all(parent).unwrap();
            }
        }
        fs::File::create(path_obj).unwrap();
    }
}
