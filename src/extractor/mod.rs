//! extract links from html source and robots.txt
mod builder;
mod container;
#[cfg(test)]
mod tests;

pub use self::builder::ExtractionTarget;
pub use self::builder::ExtractorBuilder;
pub use self::container::Extractor;

use crate::{
    config::Configuration, event_handlers::Command, scan_manager::FeroxScans, statistics::Stats,
    FeroxResponse,
};
use regex::Regex;
use std::sync::Arc;
use tokio::sync::mpsc::UnboundedSender;