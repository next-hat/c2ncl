use bollard_next::service::HealthConfig;

use crate::{
    compose::{Healthcheck, HealthcheckTest},
    utils::atoi::atoi64,
};

impl From<Healthcheck> for HealthConfig {
    fn from(value: Healthcheck) -> Self {
        HealthConfig {
            test: match value.test {
                Some(test) => match test {
                    HealthcheckTest::Multiple(multiple) => Some(multiple),
                    HealthcheckTest::Single(simple) => Some(vec![simple]),
                },
                None => None,
            },
            interval: atoi64(value.interval),
            timeout: atoi64(value.timeout),
            retries: Some(value.retries),
            start_period: atoi64(value.start_period),
        }
    }
}
