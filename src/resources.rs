use nanocl_stubs::{proxy, resource};

use crate::{compose, ports};

fn build_proxy_rule(container_name: String, port: ports::PortRedirect) -> proxy::ProxyRule {
    proxy::ProxyRule::Stream(proxy::ProxyRuleStream {
        network: "Public".to_string(),
        target: nanocl_stubs::proxy::StreamTarget::Upstream(proxy::UpstreamTarget {
            key: format!("{container_name}.global.c"),
            port: port.output,
            path: None,
            disable_logging: None,
        }),
        port: port.input,
        protocol: port.protocol,
        ssl: None,
    })
}

impl From<compose::Service> for resource::ResourcePartial {
    fn from(value: compose::Service) -> Self {
        let name = value.container_name.unwrap_or_default();
        let full_name = value.hostname.unwrap_or(name.clone());
        let parsed_ports = ports::translate_ports(value.ports).unwrap_or_default();
        let config = proxy::ResourceProxyRule {
            rules: parsed_ports
                .into_iter()
                .map(|port| build_proxy_rule(name.clone(), port))
                .collect::<Vec<proxy::ProxyRule>>(),
        };
        resource::ResourcePartial {
            name: full_name,
            kind: "ProxyRule".to_string(),
            version: "v0.7".to_string(),
            data: serde_json::json!(config),
            metadata: None,
        }
    }
}
