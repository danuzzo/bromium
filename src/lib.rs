//! # Bromium
//! 
//! Rust bindings for the Bromium project, a Python library for interacting with the WinDriver API.
//! This module provides a Python interface to the WinDriver API, allowing users to
//! automate tasks and interact with the Windows UI using Python.

mod windriver;
mod context;
mod xpath;
mod bindings;
mod commons;
mod uiauto;
mod logging;
mod app_control;

use pyo3::prelude::*;
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize logging and other global resources
fn init_bromium() {
    INIT.call_once(|| {
        // Initialize logging system
        if let Err(e) = logging::init_logging() {
            eprintln!("Failed to initialize logging: {}", e);
        }
        
        // Clean up old log files (keep last 10)
        if let Err(e) = logging::cleanup_old_logs(10) {
            log::warn!("Failed to clean up old log files: {}", e);
        }
        
        log::info!("Bromium library initialized successfully");
    });
}

/// A Python module implemented in Rust.
#[pymodule]
fn bromium(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Initialize logging and cleanup on module import
    init_bromium();
    
    m.add_class::<windriver::WinDriver>()?;
    m.add_class::<windriver::Element>()?;
    m.add_class::<context::ScreenContext>()?;
    
    log::info!("Bromium Python module loaded");
    Ok(())
}