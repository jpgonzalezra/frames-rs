pub trait HtmlSerializer {
    fn to_html(&self) -> String;
}

pub mod button;
pub mod frame;
pub mod image;
