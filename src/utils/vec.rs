#[macro_export]
macro_rules! vec_string {
    () => (
        Vec::new()
    );
    ($($element:expr),* $(,)?) => {{
        let mut temp_vec = Vec::new();
        $(temp_vec.push($element.to_string());)*
        temp_vec
    }};
}

#[macro_export]
macro_rules! vec_vec {
    () => {
        Vec::new()
    };
        ($($element:expr),* $(,)?) => {{
        let mut temp_vec = Vec::new();
        $(temp_vec.push($element.to_vec());)*
        temp_vec
    }};
}
