use std::collections::HashMap;

use crate::MetricData;

const TYPE_FIELD: &'static [u8] = "# TYPE ".as_bytes();

const COUNTER: &'static [u8] = " counter\n".as_bytes();
const GAUGE: &'static [u8] = " gauge\n".as_bytes();
const SUMMARY: &'static [u8] = " summary\n".as_bytes();

const QUANTILE_095: &'static [u8] = "{quantile=\"0.95\"} ".as_bytes();
const QUANTILE_099: &'static [u8] = "{quantile=\"0.99\"} ".as_bytes();
const QUANTILE_100: &'static [u8] = "{quantile=\"1\"} ".as_bytes();
const SUMMARY_SUM: &'static [u8] = "_sum ".as_bytes();
const SUMMARY_COUNT: &'static [u8] = "_count ".as_bytes();

const SPACE: &'static [u8] = &[32];
const NEW_LINE: &'static [u8] = &[10];

pub fn prometheus_exposition(metrics: HashMap<&'static str, MetricData>) -> Vec<u8> {
    let mut output = Vec::with_capacity(2048);

    for (key, val) in metrics {
        let key_bytes = key.as_bytes();
        output.extend(TYPE_FIELD);
        output.extend(key_bytes);

        match val {
            MetricData::Counter(x) => {
                output.extend(COUNTER);

                output.extend(key_bytes);
                output.extend(SPACE);
                output.extend(x.to_string().as_bytes());
            },
            MetricData::Gauge(x) => {
                output.extend(GAUGE);

                output.extend(key_bytes);
                output.extend(SPACE);
                output.extend(x.to_string().as_bytes());
            },
            MetricData::Duration(x) => {
                output.extend(SUMMARY);

                let count = x.len();
                let quantile_95 = (0.95f32 * (count + 1) as f32) as usize - 1;
                let quantile_99 = (0.99f32 * (count + 1) as f32) as usize - 1;

                output.extend(key_bytes);
                output.extend(QUANTILE_095);
                output.extend(x.get(quantile_95).unwrap_or(&0).to_string().as_bytes());
                output.extend(NEW_LINE);
                output.extend(key_bytes);
                output.extend(QUANTILE_099);
                output.extend(x.get(quantile_99).unwrap_or(&0).to_string().as_bytes());
                output.extend(NEW_LINE);
                output.extend(key_bytes);
                output.extend(QUANTILE_100);
                output.extend(x.get(x.len() - 1).unwrap_or(&0).to_string().as_bytes());
                output.extend(NEW_LINE);
                output.extend(key_bytes);
                output.extend(SUMMARY_SUM);
                output.extend(x.iter().sum::<u64>().to_string().as_bytes());
                output.extend(NEW_LINE);
                output.extend(key_bytes);
                output.extend(SUMMARY_COUNT);
                output.extend(count.to_string().as_bytes());
            }
        }

        output.extend(NEW_LINE);
        output.extend(NEW_LINE);
    }

    output
}