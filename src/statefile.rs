use nanocl_stubs::{cargo_spec, resource};
use serde::{Deserialize, Serialize};

use crate::compose::{ComposeFile, Service};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Statefile {
    pub api_version: String,
    pub namespace: Option<String>,
    pub resources: Option<Vec<resource::ResourcePartial>>,
    pub cargoes: Option<Vec<cargo_spec::CargoSpecPartial>>,
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

                        cargoes.push(cargo_spec::CargoSpecPartial::from(mutable_service));
                    }
                })
            })),
            _ => None,
        };

        Statefile {
            api_version: "v0.15".to_owned(),
            namespace: Some("global".to_owned()),
            cargoes: Some(cargoes),
            resources: Some(resources),
        }
    }
}
