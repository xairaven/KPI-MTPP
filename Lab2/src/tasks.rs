use crate::task::report::Reportable;
use crate::tasks::html::HtmlTags;
use thiserror::Error;

pub fn tasks() -> Vec<Box<dyn Reportable>> {
    vec![Box::new(HtmlTags::with_documents_amount(10_000))]
}

#[derive(Debug, Error)]
pub enum TaskLogicError {
    #[error("Index out of bounds: {0}")]
    IndexOutOfBounds(usize),
}

pub mod html;
