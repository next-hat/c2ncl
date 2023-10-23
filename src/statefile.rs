use nanocl_stubs::{cargo_config, resource};
use serde::{Deserialize, Serialize};

use crate::compose::{ComposeFile, Service};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Statefile {
    pub kind: String,
    pub api_version: String,
    pub namespace: Option<String>,
    pub resources: Option<Vec<resource::ResourcePartial>>,
    pub cargoes: Option<Vec<cargo_config::CargoConfigPartial>>,
}

impl From<ComposeFile> for Statefile {
    fn from(value: ComposeFile) -> Self {
        let mut cargoes = Vec::new();
        let mut resources = Vec::new();
        match value {
            ComposeFile::V2Plus(compose) => Some(compose.services.map(|services| {
                services.0.into_iter().for_each(|(name, service)| {
                    if let Some(s) = service {
                        let mut mutable_service: Service = s.clone();

                        if s.container_name.is_none() {
                            mutable_service.container_name = Some(name);
                        } else {
                            resources.push(resource::ResourcePartial::from(s));
                        }

                        cargoes.push(cargo_config::CargoConfigPartial::from(mutable_service));
                    }
                })
            })),
            _ => None,
        };

        Statefile {
            kind: "Deployment".to_owned(),
            api_version: "v0.10".to_owned(),
            namespace: Some("global".to_owned()),
            cargoes: Some(cargoes),
            resources: Some(resources),
        }
    }
}
