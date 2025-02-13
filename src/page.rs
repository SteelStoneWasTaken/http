pub fn get(path: &str) -> String {

    let path: Vec<&str> = path.split("/").collect();

    match path[1] {
        "home" => {
            return format!(r#"
                asdsadasd {}
            "#,
                path.join("/")
            );
        }
        _ => ()
    }

    return format!(r#"
        <h1> 404, Not found: </h1>
        {}
    "#,
        path.join("/")
    );
}