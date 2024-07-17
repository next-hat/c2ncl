use std::collections::HashMap;

use nanocl_stubs::cargo_spec::{CargoSpecPartial, Config, HostConfig};

use crate::{
    compose,
    utils::{
        atoi::atoi64,
        options::{option_fold_empty_object, option_into},
    },
};

impl From<compose::Environment> for Vec<String> {
    fn from(value: compose::Environment) -> Self {
        match value {
            compose::Environment::KvPair(map) => map
                .iter()
                .map(|(key, value)| format!("{key}={}", value.clone().unwrap_or_default()))
                .collect(),
            compose::Environment::List(v) => v,
        }
    }
}

impl From<compose::Service> for Config {
    fn from(config: compose::Service) -> Config {
        Config {
            hostname: config.hostname,
            user: config.user,
            exposed_ports: option_fold_empty_object(config.expose),
            tty: config.tty,
            open_stdin: Some(config.stdin_open),
            env: option_into(config.environment),
            cmd: config.command.map(|cmd| match cmd {
                compose::Command::Simple(command) => {
                    command.split(' ').map(|str| str.to_owned()).collect()
                }
                compose::Command::Args(args) => args,
            }),
            healthcheck: option_into(config.healthcheck),
            image: config.image,
            host_config: Some(HostConfig {
                binds: config.volumes.map(|volumes| {
                    volumes
                        .into_iter()
                        .map(|volume| match volume {
                            compose::Volume::Simple(v) => v,
                            compose::Volume::Advanced(advanced_volume) => {
                                format!("{}:{}", advanced_volume.source, advanced_volume.target)
                            }
                        })
                        .collect()
                }),
                ..Default::default()
            }),
            working_dir: config.working_dir,
            entrypoint: match config.entrypoint {
                Some(entrypoint) => match entrypoint {
                    compose::Entrypoint::List(list) => Some(list),
                    compose::Entrypoint::Simple(simple) => Some(vec![simple]),
                },
                None => None,
            },
            labels: match config.labels {
                Some(labels) => match labels.0 {
                    compose::Label::KvPair(map) => Some(map),
                    compose::Label::List(list) => {
                        Some(list.into_iter().fold(HashMap::new(), |mut acc, label| {
                            let splits = label.split('=').collect::<Vec<&str>>();
                            let name = splits.first();

                            match name {
                                Some(split) => {
                                    let value = if splits.len() > 1 {
                                        splits[1..].join("=")
                                    } else {
                                        "".to_string()
                                    };

                                    acc.insert(split.to_string(), value);

                                    acc
                                }
                                None => acc,
                            }
                        }))
                    }
                },

                None => None,
            },
            stop_signal: config.stop_signal,
            //TODO: CHECK
            stop_timeout: atoi64(config.stop_grace_period),
            ..Default::default()
        }
    }
}

impl From<compose::Service> for CargoSpecPartial {
    fn from(value: compose::Service) -> Self {
        CargoSpecPartial {
            replication: None,
            container: value.clone().into(),
            name: value.container_name.unwrap_or_default(),
            ..Default::default()
        }
    }
}
