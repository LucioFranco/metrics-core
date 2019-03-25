use metrics_core::MetricsObserver;

pub struct PrometheusRecorder {
    output: String,
}

impl PrometheusRecorder {
    pub fn new() -> Self {
        PrometheusRecorder {
            output: String::new(),
        }
    }
}

impl MetricsObserver for PrometheusRecorder {
    fn observe_counter<K: AsRef<str>>(&mut self, label: K, value: u64) {
        let label = label.as_ref().replace('.', "_");
        self.output.push_str("# TYPE ");
        self.output.push_str(label.as_str());
        self.output.push_str(" counter\n");
        self.output.push_str(label.as_str());
        self.output.push_str(" ");
        self.output.push_str(value.to_string().as_str());
        self.output.push_str("\n");
    }

    fn observe_gauge<K: AsRef<str>>(&mut self, label: K, value: i64) {
        let label = label.as_ref().replace('.', "_");
        self.output.push_str("# TYPE ");
        self.output.push_str(label.as_str());
        self.output.push_str(" gauge\n");
        self.output.push_str(label.as_str());
        self.output.push_str(" ");
        self.output.push_str(value.to_string().as_str());
        self.output.push_str("\n");
    }

    fn observe_histogram<K: AsRef<str>>(&mut self, label: K, quantiles: &[(f64, u64)]) {
        let label = label.as_ref().replace('.', "_");
        self.output.push_str("# TYPE ");
        self.output.push_str(label.as_str());
        self.output.push_str(" summary\n");
        for (percentile, value) in quantiles {
            self.output.push_str(label.as_str());
            self.output.push_str("{quantile=\"");
            self.output.push_str(percentile.to_string().as_str());
            self.output.push_str("\"} ");
            self.output.push_str(value.to_string().as_str());
            self.output.push_str("\n");
        }
        self.output.push_str(label.as_str());
        self.output.push_str("_sum ");
        self.output.push_str(summary.sum().to_string().as_str());
        self.output.push_str("\n");
        self.output.push_str(label.as_str());
        self.output.push_str("_count ");
        self.output.push_str(summary.count().to_string().as_str());
        self.output.push_str("\n");
    }
}
