//! Foundational traits for interoperable metrics libraries in Rust.
//!
//! # Common Ground
//! Most libraries, under the hood, are all based around a core set of data types: counters,
//! gauges, and histograms.  While the API surface may differ, the underlying data is the same.
//!
//! # Metric Types
//!
//! ## Counters
//! Counters represent a single value that can only ever be incremented over time, or reset to
//! zero.
//!
//! Counters are useful for tracking things like operations completed, or errors raised, where
//! the value naturally begins at zero when a process or service is started or restarted.
//!
//! ## Gauges
//! Gauges represent a single value that can go up _or_ down over time.
//!
//! Gauges are useful for tracking things like the current number of connected users, or a stock
//! price, or the temperature outside.
//!
//! ## Histograms
//! Histograms measure the distribution of values for a given set of measurements.
//!
//! Histograms are generally used to derive statistics about a particular measurement from an
//! operation or event that happens over and over, such as the duration of a request, or number of
//! rows returned by a particular database query.
//!
//! Histograms allow you to answer questions of these measurements, such as:
//! - "What were the fastest and slowest requests in this window?"
//! - "What is the slowest request we've seen out of 90% of the requests measured? 99%?"
//!
//! Histograms are a convenient way to measure behavior not only at the median, but at the edges of
//! normal operating behavior.

/// A value that observes collected metrics.
pub trait MetricsObserver {
    /// Observes a counter.
    ///
    /// From the perspective of an observer, a counter and gauge are essentially identical, insofar
    /// as they are both a single value tied to a key.  From the perspective of a collector,
    /// counters and gauges usually have slightly different modes of operation.
    ///
    /// For the sake of flexibility on the observer side, both are provided.
    fn observe_counter<K: AsRef<str>>(&mut self, key: K, value: u64);

    /// Observes a gauge.
    ///
    /// From the perspective of a observer, a counter and gauge are essentially identical, insofar
    /// as they are both a single value tied to a key.  From the perspective of a collector,
    /// counters and gauges usually have slightly different modes of operation.
    ///
    /// For the sake of flexibility on the observer side, both are provided.
    fn observe_gauge<K: AsRef<str>>(&mut self, key: K, value: i64);

    /// Observes a histogram.
    ///
    /// The quantiles are expected to be in the format of (quantile, value_at_quantile).
    ///
    /// Quantiles are a value from 0 to 1, inclusive.  Values outside of this range are considered
    /// invalid, but it is an implementation detail of the observer on how to handle them.
    fn observe_histogram<K: AsRef<str>>(&mut self, key: K, quantiles: &[(f64, u64)]);
}
