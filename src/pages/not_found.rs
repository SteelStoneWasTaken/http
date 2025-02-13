pub fn get(path: Vec<&str>) -> String {
    return format!(r#"
        <h1> 404, Not found: </h1>
        {}
    "#,
        path.join("/")
    );
}