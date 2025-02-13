mod index;
mod not_found;

pub fn get(path: &str) -> String {

    let mut path: Vec<&str> = path.split("/").collect();
    path.push("");

    match path[1] {
        "" => {
            return index::get();
        }
        "api" => {
            match path[2] {
                "" => {
                    return format!(r#"
                        <link rel="stylesheet" href="style/default.css">
                        Api
                    "#
                    );
                }
                _ => ()
            }
        }
        _ => {}
    }

    return not_found::get(path);
}