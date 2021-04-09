lazy_static::lazy_static! {
    pub static ref TERA: tera::Tera = {
        let tera = tera::Tera::new("templates/*.tera").unwrap();
        tera
    };
}
