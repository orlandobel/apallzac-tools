use log::error;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PrinterInfo {
    pub name: String,
    pub is_default: bool,
}

/// Lists all physical printers available on the system.
#[tauri::command]
pub fn get_printers() -> Result<Vec<PrinterInfo>, String> {
    #[cfg(windows)]
    return get_printers_windows();

    #[cfg(not(windows))]
    return get_printers_cups();
}

/// Sends a PDF file to the specified printer with default settings:
/// A4 paper, color, 1 copy.
#[tauri::command]
pub fn print_pdf_file(printer: String, path: String) -> Result<(), String> {
    #[cfg(windows)]
    return print_pdf_windows(&printer, &path);

    #[cfg(not(windows))]
    return print_pdf_cups(&printer, &path);
}

// ─── Windows ─────────────────────────────────────────────────────────────────

#[cfg(windows)]
fn get_printers_windows() -> Result<Vec<PrinterInfo>, String> {
    use std::process::Command;

    let ps_script = r#"
        $printers = Get-Printer | Select-Object `
            @{Name="name";Expression={$_.Name}}, `
            @{Name="is_default";Expression={[bool]$_.Default}}
        $printers | ConvertTo-Json -Compress
    "#;

    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", ps_script])
        .output()
        .map_err(|e| {
            let err = e.to_string();
            println!("PrinterUtils@get_printers_windows - Error :: {}", err);
            error!("PrinterUtils@get_printers_windows - Error getting printers :: {}", e);
            err
        })?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let json_str = json_str.trim();

    if json_str.is_empty() {
        return Ok(vec![]);
    }

    // PowerShell returns a bare object (not array) when there is only one printer
    if json_str.starts_with('[') {
        serde_json::from_str(json_str).map_err(|e| e.to_string())
    } else if json_str.starts_with('{') {
        let single: PrinterInfo = serde_json::from_str(json_str).map_err(|e| e.to_string())?;
        Ok(vec![single])
    } else {
        Ok(vec![])
    }
}

#[cfg(windows)]
fn print_pdf_windows(printer: &str, path: &str) -> Result<(), String> {
    use std::process::Command;

    // Escape single quotes for PowerShell string literals
    let safe_printer = printer.replace('\'', "''");
    let safe_path = path.replace('\'', "''");

    let script = format!(
        "Start-Process -FilePath '{safe_path}' -Verb 'printto' -ArgumentList '{safe_printer}' -Wait"
    );

    let output = Command::new("powershell")
        .args(["-NoProfile", "-NonInteractive", "-Command", &script])
        .output()
        .map_err(|e| {
            let err = e.to_string();
            println!("PrinterUtils@get_printers_windows - Error :: {}", err);
            error!("PrinterUtils@print_pdf_windows - Error printing PDF :: {}", e);
            err
        })?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// ─── Linux / macOS (CUPS) ─────────────────────────────────────────────────────

#[cfg(not(windows))]
fn get_printers_cups() -> Result<Vec<PrinterInfo>, String> {
    use std::process::Command;
    println!("PrinterUtils@get_printers_cups - get_printers_cups");

    // Resolve the current default printer name
    let default_name = {
        let out = Command::new("lpstat").args(["-d"]).output().ok();
        let name = out.and_then(|o| {
            let s = String::from_utf8_lossy(&o.stdout).to_string();
            // "system default destination: <name>"
            s.trim()
                .rsplit(": ")
                .next()
                .map(|n| n.trim().to_string())
        });
        if name.is_none() {
            error!("PrinterUtils@get_printers_cups - Failed to get default printer name from lpstat");
        }
        name.unwrap_or_default()
    };

    println!("PrinterUtils@get_printers_cups - default_name: {}", default_name);

    // List all accepting queues: "<name> accepting requests since ..."
    let output = Command::new("lpstat")
        .args(["-a"])
        .output()
        .map_err(|e| {
            println!("PrinterUtils@get_printers_cups - error :: {:?}", e);
            error!("PrinterUtils@get_printers_cups - Failed to execute lpstat :: {}", e);
            e.to_string()
        })?;

    println!("PrinterUtils@get_printers_cups - output :: {:?}", output);

    let stdout = String::from_utf8_lossy(&output.stdout);

    let printers: Vec<PrinterInfo> = stdout
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|line| {
            let name = line.split_whitespace().next()?.to_string();
            Some(PrinterInfo {
                is_default: name == default_name,
                name,
            })
        })
        .collect();

    println!("PrinterUtils@get_printers_cups - printers :: {:?}", printers);

    Ok(printers)
}

#[cfg(not(windows))]
fn print_pdf_cups(printer: &str, path: &str) -> Result<(), String> {
    use std::process::Command;

    // -P  : destination printer
    // -o media=A4   : A4 paper
    // -o ColorModel=Color : color output
    // -# 1 : 1 copy
    let output = Command::new("lpr")
        .args([
            "-P",
            printer,
            "-o",
            "media=A4",
            "-o",
            "ColorModel=Color",
            "-#",
            "1",
            path,
        ])
        .output()
        .map_err(|e| { 
            let err = e.to_string();
            println!("PrinterUtils@get_printers_windows - Error :: {}", err);
            error!("PrinterUtils@print_pdf_cups - Error printing PDF :: {}", e);
            err
        })?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Err(if !stderr.is_empty() { stderr } else { stdout })
    }
}
