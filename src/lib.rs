//! Pipe viewer is a terminal-based tool for monitoring the progress of data through a pipeline

pub mod args;
pub mod read;
pub mod stats;
pub mod write;

pub const CHUNK_SIZE: usize = 16 * 1024;
