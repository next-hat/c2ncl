use nanocl_stubs::proxy::ProxyStreamProtocol;
use regex::Regex;

use crate::{
    compose::{self, Port},
    utils::atoi::atoi,
};

const IP_REGEXP: &str = r"(\[?(?P<host>[a-fA-F\d.:]+)\]?:)?";
const EXT_PORT_REGEXP: &str = r"(?P<ext>[\d]*)(-(?P<ext_end>[\d]+))?:";
const INT_PORT_REGEXP: &str = r"(?P<int>[\d]+)(-(?P<int_end>[\d]+))?";
const PROTO_REGEXP: &str = r"(/(?P<proto>(udp|tcp|sctp)))?";

pub struct PortRedirect {
    pub input: u16,
    pub output: u16,
    pub protocol: ProxyStreamProtocol,
}

pub fn translate_ports(ports: Option<Vec<compose::Port>>) -> Option<Vec<PortRedirect>> {
    match ports {
        None => None,
        ports => ports
            .unwrap()
            .into_iter()
            .map(|port| match port {
                Port::Simple(simple_port) => {
                    let string_port = simple_port.to_string();

                    Some(PortRedirect {
                        input: atoi::<u16>(Some(string_port.clone())).unwrap(),
                        output: atoi::<u16>(Some(string_port)).unwrap(),
                        protocol: ProxyStreamProtocol::Tcp,
                    })
                }
                Port::Parsed(parsed_port) => Some(parsed_port.into()),
            })
            .collect(),
    }
}

impl From<String> for PortRedirect {
    fn from(value: String) -> Self {
        let parsed_port_regex: String =
            format!("^({IP_REGEXP}{EXT_PORT_REGEXP})?{INT_PORT_REGEXP}{PROTO_REGEXP}$");

        let parsed_port_to_exposed_ports: Regex = Regex::new(parsed_port_regex.as_str()).unwrap();
        let cap = parsed_port_to_exposed_ports
            .captures_iter(value.as_str())
            .next();

        cap.map(|capture| {
            if capture.name("int_end").is_some() || capture.name("ext_end").is_some() {
                eprintln!("Port ranges are not supported");
            }

            if capture.name("host").is_some() {
                eprintln!("Port ranges are not supported");
            }
            let protocol = match capture.name("proto") {
                Some(proto) => {
                    match proto.as_str() {
                        "udp" => ProxyStreamProtocol::Udp,
                        // "sctp" => ProxyStreamProtocol::Sctp,
                        _ => ProxyStreamProtocol::Tcp,
                    }
                }
                None => ProxyStreamProtocol::Tcp,
            };
            let input_port = capture.name("int").unwrap().as_str().to_owned();
            let output_port = capture
                .name("out")
                .unwrap_or(capture.name("int").unwrap())
                .as_str()
                .to_owned();
            PortRedirect {
                input: atoi::<u16>(Some(input_port)).unwrap(),
                output: atoi::<u16>(Some(output_port)).unwrap(),
                protocol,
            }
        })
        .unwrap()
    }
}
