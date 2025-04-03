use crate::distribution::Distribution;

pub struct SamplingTask<'a> {
    label: String,
    num_sampling_attempts: u64,
    distribution: &'a dyn Distribution,
}

impl<'a> SamplingTask<'a> {
    pub fn new(
        distribution: &'a dyn Distribution,
        num_sampling_attempts: u64,
        label: &str,
    ) -> Self {
        SamplingTask {
            label: label.into(),
            num_sampling_attempts,
            distribution,
        }
    }

    pub fn get_distribution(&self) -> &dyn Distribution {
        self.distribution
    }

    pub fn get_label(&self) -> &str {
        &self.label
    }

    pub fn get_num_sampling_attempts(&self) -> u64 {
        self.num_sampling_attempts
    }
}
