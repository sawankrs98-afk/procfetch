// CPU STATISTICS
// fetches the raw cpu information from the file /proc/cpuinfo
use std::fs;
use std::io;

#[derive(Debug, Clone)]
pub struct cpuinfo {
    pub model_name: String,
    pub logical_cores: usize,
    pub mhz: Option<f64>,
}
// see cat /proc/cpuinfo for the details about your cpu...
pub fn fetch_cpu_info() -> Result<cpuinfo, io::Error> {
    let content: String = fs::read_to_string("/proc/cpuinfo")?;

    // init all the variables for the data fetching...
    let mut model_name: Option<String> = None;
    let mut logical_cores: usize = 0;
    let mut mhz: Option<f64> = None;

    // now reading the content line by lines..
    for line in content.lines() {
        if line.starts_with("processor") {
            logical_cores += 1;
            continue;
        }

        if model_name.is_none() && line.starts_with("model name") || line.starts_with("Hardware") {
            if let Some((_key, value)) = line.split_once(':') {
                model_name = Some(value.trim().to_string());
            }
        }

        if mhz.is_none() && line.starts_with("cpu MHz") {
            if let Some((_key, value)) = line.split_once(':') {
                mhz = value.trim().parse::<f64>().ok();
            }
        }
    }

    let cores = if logical_cores > 0 { logical_cores } else { 1 };
    let final_model = model_name.unwrap_or_else(|| "Unknown CPU".to_string());

    Ok(cpuinfo {
        model_name: final_model,
        logical_cores: cores,
        mhz: mhz,
    })
}
