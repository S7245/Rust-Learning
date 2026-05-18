// Day 01 - FerrisTunnel workspace foundation in one standalone file.
//
// Run:
// rustc Days/Day01/Day01_workspace_foundation.rs -o /tmp/day01 && /tmp/day01

mod ferris_tunnel_core {
    use std::net::IpAddr;

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct TunnelProfile {
        name: String,
        routes: Vec<Route>,
    }

    impl TunnelProfile {
        pub fn new(name: impl Into<String>) -> Self {
            Self {
                name: name.into(),
                routes: Vec::new(),
            }
        }

        pub fn add_route(&mut self, route: Route) {
            self.routes.push(route);
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn routes(&self) -> &[Route] {
            &self.routes
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Route {
        destination: IpAddr,
        prefix: u8,
        peer_name: String,
    }

    impl Route {
        pub fn new(destination: IpAddr, prefix: u8, peer_name: impl Into<String>) -> Self {
            Self {
                destination,
                prefix,
                peer_name: peer_name.into(),
            }
        }

        pub fn destination(&self) -> IpAddr {
            self.destination
        }

        pub fn prefix(&self) -> u8 {
            self.prefix
        }

        pub fn peer_name(&self) -> &str {
            &self.peer_name
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TunnelState {
        Disconnected,
        Ready,
    }

    #[derive(Debug)]
    pub struct TunnelSummary<'a> {
        pub profile_name: &'a str,
        pub route_count: usize,
        pub state: TunnelState,
    }

    pub fn summarize(profile: &TunnelProfile, state: TunnelState) -> TunnelSummary<'_> {
        TunnelSummary {
            profile_name: profile.name(),
            route_count: profile.routes().len(),
            state,
        }
    }
}

mod ferris_tunnel_net {
    use std::net::{IpAddr, Ipv4Addr, SocketAddr};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TransportKind {
        Tcp,
        Udp,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ListenerConfig {
        bind: SocketAddr,
        transport: TransportKind,
    }

    impl ListenerConfig {
        pub fn local_demo(transport: TransportKind) -> Self {
            Self {
                bind: SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), 9000),
                transport,
            }
        }

        pub fn bind(&self) -> SocketAddr {
            self.bind
        }

        pub fn transport(&self) -> TransportKind {
            self.transport
        }
    }
}

mod ferris_tunnel_cli {
    use crate::ferris_tunnel_core::{summarize, Route, TunnelProfile, TunnelState};
    use crate::ferris_tunnel_net::{ListenerConfig, TransportKind};
    use std::net::{IpAddr, Ipv4Addr};

    pub fn run_demo() {
        let mut profile = TunnelProfile::new("local-dev");

        let route = Route::new(
            IpAddr::V4(Ipv4Addr::new(10, 8, 0, 0)),
            24,
            "edge-peer",
        );
        profile.add_route(route);

        let tcp_listener = ListenerConfig::local_demo(TransportKind::Tcp);
        let udp_listener = ListenerConfig::local_demo(TransportKind::Udp);
        let initial = summarize(&profile, TunnelState::Disconnected);
        let summary = summarize(&profile, TunnelState::Ready);

        println!("FerrisTunnel workspace foundation");
        println!("profile      : {}", summary.profile_name);
        println!("routes       : {}", summary.route_count);
        println!("initial state: {:?}", initial.state);
        println!("state        : {:?}", summary.state);
        println!(
            "tcp listener : {:?}://{}",
            tcp_listener.transport(),
            tcp_listener.bind()
        );
        println!(
            "udp listener : {:?}://{}",
            udp_listener.transport(),
            udp_listener.bind()
        );

        for route in profile.routes() {
            println!(
                "route        : {}/{} via {}",
                route.destination(),
                route.prefix(),
                route.peer_name()
            );
        }
    }
}

fn main() {
    ferris_tunnel_cli::run_demo();
}
