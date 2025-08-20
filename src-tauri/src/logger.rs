use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use chrono::{DateTime, Utc};
use serde_json::json;

pub struct Logger {
    log_dir: String,
}

impl Logger {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let log_dir = "../logs".to_string();
        
        // Create logs directory if it doesn't exist
        if !Path::new(&log_dir).exists() {
            fs::create_dir_all(&log_dir)?;
        }
        
        Ok(Logger { log_dir })
    }
    
    pub fn log_info(&self, component: &str, message: &str, data: Option<serde_json::Value>) {
        self.write_log("INFO", component, message, data);
    }
    
    pub fn log_error(&self, component: &str, message: &str, data: Option<serde_json::Value>) {
        self.write_log("ERROR", component, message, data);
        eprintln!("[ERROR] {}: {}", component, message);
    }
    
    pub fn log_debug(&self, component: &str, message: &str, data: Option<serde_json::Value>) {
        self.write_log("DEBUG", component, message, data);
    }
    
    pub fn log_warn(&self, component: &str, message: &str, data: Option<serde_json::Value>) {
        self.write_log("WARN", component, message, data);
    }
    
    fn write_log(&self, level: &str, component: &str, message: &str, data: Option<serde_json::Value>) {
        let timestamp = Utc::now();
        let date_str = timestamp.format("%Y-%m-%d").to_string();
        let log_file = format!("{}/ligeia-backend-{}.log", self.log_dir, date_str);
        
        let log_entry = json!({
            "timestamp": timestamp.to_rfc3339(),
            "level": level,
            "component": component,
            "message": message,
            "data": data
        });
        
        let log_line = format!("{}\n", log_entry.to_string());
        
        // Print to console as well
        println!("[{}] {}: {}", level, component, message);
        
        // Write to file
        if let Ok(mut file) = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file) {
            let _ = file.write_all(log_line.as_bytes());
            let _ = file.flush();
        }
    }
}