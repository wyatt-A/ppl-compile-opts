use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use serde::Deserialize;

#[cfg(test)]
mod tests {
    use crate::Options;

    #[test]
    fn load_opts() {
        let opts = Options::from_file("C:/Users/MRS/ppl-compile-opts/ppl_compile_opts.toml");
    }

}

#[derive(Debug, Clone, Deserialize)]
pub struct Options {
    pub clock: Clock,
    pub event_timing: EventTiming,
    pub dac: Dac,
    pub system_vars: SystemVars,
    pub limits: Limits
}

#[derive(Debug, Clone, Deserialize, Default)]
pub struct SystemVars {
    pub parfilio_path: PathBuf,
    pub seq_gen_path: PathBuf,
    pub ppl_compiler_path: PathBuf,
    pub seq_gen_rf_template: PathBuf,
    pub seq_gen_grad_template: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Clock {
    /// master pulse program time base
    pub clock_period_ns: usize,
    /// min clocks per rf sample
    pub min_rf_clocks_per_sample: usize,
    /// min clocks per grad sample
    pub min_grad_clocks_per_sample: usize,
    /// minimum valid delay in clocks
    pub min_delay_clocks: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct EventTiming {
    /// total delay between event start and rf pulse
    pub rf_schedule_delay_clocks: usize,
    /// part of total delay attributed to the rfstart command itself
    pub rf_lag_clocks: usize,
    /// delay after rf pulse concludes before control is returned
    pub rf_return_delay_clocks: usize,
    /// delay before start of gradient ramp
    pub grad_sched_delay_clocks: usize,
    /// time for control to return after grad start command
    pub grad_ret_delay_clocks: usize,
    /// total delay before start of sample acquisition
    pub acq_sched_delay_clocks: usize,
    /// part of the total delay attributed to the call to acquire
    pub acq_lag_clocks: usize,
    /// delay between last sample and control return
    pub acq_return_delay_clocks_1: usize,
    /// delay between last sample and control return
    pub acq_return_delay_clocks_2: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Dac {
    /// max dac value of rf power
    pub dac_rf_max: i32,
    /// max dac value of gradients
    pub dac_grad_max: i32,
    /// converts degrees to dac
    pub dac_phase_res_deg: f64,
}


#[derive(Debug, Clone, Deserialize)]
pub struct Limits {
    pub max_lut_i16_entries: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_lut_i16_entries: 196095
        }
    }
}

impl Options {

    pub fn load() -> Options {
        Self::from_file("C:/Users/MRS/ppl-compile-opts/ppl_compile_opts.toml")
        //Self::from_file("/Users/Wyatt/ppl-compile-opts/ppl_compile_opts.toml")
        //Self::from_file("/home/wyatt/ppl-compile-opts/ppl_compile_opts.toml")
        //Self::from_file("C:/Users/waust/ppl-compile-opts/ppl_compile_opts.toml")
    }

    pub fn from_file(conf_file: impl AsRef<Path>) -> Options {
        let mut f = File::open(&conf_file).expect(&format!("failed to open {}",conf_file.as_ref().display()));
        let mut s = String::new();
        f.read_to_string(&mut s).expect("failed to read string");
        match toml::from_str::<Options>(&s) {
            Ok(opts) => opts,
            Err(e) => {
                panic!("failed to parse toml: {:?}", e);
            }
        }
    }
}

impl Default for Clock {
    fn default() -> Clock {
        Clock {
            clock_period_ns: 100,
            min_rf_clocks_per_sample: 20,
            min_grad_clocks_per_sample: 20,
            min_delay_clocks: 20,
        }
    }
}

impl Default for EventTiming {
    fn default() -> EventTiming {
        EventTiming {
            rf_schedule_delay_clocks: 1300,
            rf_lag_clocks: 500,
            rf_return_delay_clocks: 50,
            grad_sched_delay_clocks: 50,
            grad_ret_delay_clocks: 50,
            acq_sched_delay_clocks: 1000,
            acq_lag_clocks: 880,
            acq_return_delay_clocks_1: 600,
            acq_return_delay_clocks_2: 600,
        }
    }
}
