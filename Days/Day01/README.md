# Day 01｜Rust 工程世界观与 Cargo Workspace：从单文件到系统网络项目骨架

## 1. 概念剖析 (The Why and What)

### Rust 学习的第一步不是语法，而是工程边界

很多客户端开发者学习一门新语言时，会先从变量、函数、控制流开始。这当然重要，但 Rust 的学习顺序要稍微换一下：你需要尽早理解它如何组织一个真实工程。

原因很简单：Rust 不是只靠语法提供安全性。它把很多工程约束都放进了编译模型里：

- 一个 package 如何被编译
- 一个 crate 暴露哪些 API
- 一个 module 默认是否私有
- 一个 workspace 如何组织多个 crate
- 哪些边界应该稳定，哪些实现细节应该隐藏

在 FerrisTunnel 这种系统网络项目里，边界尤其重要。你会同时面对 CLI、配置、协议、路由、网络 I/O、异步任务、观测性和安全策略。如果第一天就把所有代码堆在一个文件里，后面会很快失控。

### package、crate、workspace 分别是什么

先记住三个词：

| Cargo 概念 | 含义 | 类比 |
| --- | --- | --- |
| package | 一个带 `Cargo.toml` 的发布/构建单元 | 一个 Xcode project 或 Swift package |
| crate | Rust 的编译单元，可以是 library 或 binary | 一个 framework/library target 或 app target |
| workspace | 管理多个 package 的顶层工程 | Xcode workspace + Swift Package 多 target 组合 |

一个常见系统网络项目可以这样拆：

```text
ferris-tunnel/
  Cargo.toml                 # workspace root
  crates/
    ferris_tunnel_core/       # 领域模型：Profile、Route、Session
    ferris_tunnel_net/        # 网络边界：TCP/UDP/TUN 抽象
    ferris_tunnel_cli/        # CLI 入口：解析命令、调用 app service
```

第一天不需要把所有 crate 都写完整。你只需要建立一个核心认知：Rust 的工程边界不是目录装饰，而是编译、可见性和 API 稳定性的边界。

### Swift / Objective-C 对比映射

| Rust 工程概念 | Swift / Objective-C 近似概念 | 关键差异 |
| --- | --- | --- |
| `Cargo.toml` | `Package.swift` / Xcode Build Settings | Cargo 同时管理依赖、feature、edition、workspace 成员 |
| binary crate | App target / command line target | `main.rs` 是程序入口 |
| library crate | Framework / Swift package target | `lib.rs` 是库入口，默认私有，显式 `pub` 才暴露 |
| `mod` | 文件/命名空间组织 | Rust module 默认私有，不像 Objective-C header 默认倾向暴露 |
| `pub` | public API / header 声明 | Rust 要你主动选择哪些能力可被外部调用 |
| workspace | Xcode workspace | Cargo workspace 会统一 lockfile、target 目录和成员依赖解析 |

Swift 和 Objective-C 里，很多边界来自约定：文件夹、target、header、access control。Rust 也有这些概念，但它更严格：默认私有，默认不暴露，默认让你先想清楚 API 边界。

### Day1 要建立的项目心智

FerrisTunnel 的长期目标是本地 VPN/代理式网络隧道。它后面会涉及 TCP/UDP 转发、路由表、会话表、加密握手和异步 worker。Day1 只做一件事：把工程拆成三种职责。

```text
core: 只表达业务和系统网络领域模型
net : 只表达网络传输边界
cli : 只表达命令行入口和用户交互
```

这个拆法会贯穿整个课程。今天你先用一个单文件 Rust 程序模拟这三个 crate；真正进入 Cargo workspace 时，再把这些 module 拆成独立 package。

---

## 2. 核心代码演示 (The How)

下面是一份可以直接作为 `main.rs` 运行的示例。它用一个文件模拟未来的 workspace 分层：

- `ferris_tunnel_core`：领域模型
- `ferris_tunnel_net`：网络配置和 listener 描述
- `ferris_tunnel_cli`：CLI 输出和演示入口

你可以直接运行：

```bash
rustc Days/Day01/Day01_workspace_foundation.rs -o /tmp/day01 && /tmp/day01
```

完整代码：

```rust
// Day 01 - FerrisTunnel workspace foundation in one standalone file.
//
// 在真实项目里，这三个 module 会拆成三个 crate：
// - ferris_tunnel_core
// - ferris_tunnel_net
// - ferris_tunnel_cli
//
// Day1 先用单文件模拟边界，方便在 CodeSandbox 或 rustc 中直接运行。

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
```

### 这段代码在训练什么

第一，`core` 不依赖 `cli`。领域模型不应该知道命令行怎么打印，也不应该知道用户从哪里输入参数。

第二，`net` 不直接控制真实 socket。Day1 只建模 `ListenerConfig` 和 `TransportKind`，避免一上来就陷入异步网络细节。

第三，`cli` 是组合层。它可以依赖 `core` 和 `net`，把 profile、route、listener 组装成用户能看到的输出。

这就是 workspace 的第一层价值：让依赖方向保持单向。

---

## 3. 架构师的实战洞察

### VPN / 网络隧道中的落地场景

一个 VPN/代理项目如果不拆边界，很容易变成这样：

```text
main.rs 同时负责：
配置解析 + 路由表 + socket + 加密 + 日志 + CLI 输出
```

短期看很快，长期会非常难改。比如你想把 UDP transport 换成 TUN/TAP，或者想把 CLI 换成桌面端调用的 SDK，如果一开始没有 crate 边界，后面会被迫大规模重构。

更稳的方向是：

```text
ferris_tunnel_core  : Profile, Route, Session, TunnelState
ferris_tunnel_net   : Transport, Listener, Packet I/O
ferris_tunnel_crypto: Handshake, Key, Cipher
ferris_tunnel_cli   : Command parsing, terminal output
```

Day1 只做前三个模块的雏形，重点是建立工程秩序。

### 虚拟币加密通信中的落地场景

加密通信系统通常要把协议、密钥、传输、业务命令分开。如果密钥逻辑散落在 CLI 或 socket 代码里，审计会变得困难。Rust 的 crate 和 module 边界可以帮你把“哪些 API 可被外部调用”控制得很窄。

安全系统里，`pub` 越多，审计面越大。Day1 开始就要养成默认私有、按需暴露的习惯。

### 量化交易高并发架构中的落地场景

量化交易系统里，行情接入、订单簿、策略、风控、下单通道也需要清晰边界。否则一个策略改动可能影响行情解析，一个日志改动可能拖慢热路径。

Rust workspace 的好处是可以把热路径 crate 独立出来，单独 benchmark、单独审查依赖、单独控制 feature。

### 避坑指南

| 错误模式 | 为什么危险 | 正确做法 |
| --- | --- | --- |
| 一开始把所有代码写进 `main.rs` | 后续网络、协议、CLI、测试都会耦合 | 先用 module 建边界，再拆 crate |
| 到处 `pub` | 内部实现变成外部契约，后面不敢改 | 默认私有，只暴露稳定 API |
| `core` 依赖 `cli` | 领域模型被用户界面污染 | 依赖方向保持 `cli -> app/net/core` |
| Day1 就接真实网络 | 学习焦点会从工程边界跑偏 | 先建配置和模型，再进入 socket |
| 把 workspace 当目录分类 | workspace 是构建和依赖边界，不只是文件夹 | 用 crate 边界表达长期架构 |

---

## 4. 今日挑战

### 题目：把单文件设计改造成真实 Cargo Workspace

在本地或 CodeSandbox 中创建一个最小 workspace：

```text
ferris-tunnel/
  Cargo.toml
  crates/
    ferris_tunnel_core/
    ferris_tunnel_net/
    ferris_tunnel_cli/
```

目标：

1. workspace root 的 `Cargo.toml` 声明三个 members。
2. `ferris_tunnel_core` 是 library crate，提供 `TunnelProfile`、`Route`、`TunnelState`。
3. `ferris_tunnel_net` 是 library crate，提供 `TransportKind`、`ListenerConfig`。
4. `ferris_tunnel_cli` 是 binary crate，依赖前两个 crate，并打印 demo 输出。
5. 运行 `cargo run -p ferris_tunnel_cli` 能看到 profile、route、listener 信息。

### Hints

workspace root：

```toml
[workspace]
members = [
  "crates/ferris_tunnel_core",
  "crates/ferris_tunnel_net",
  "crates/ferris_tunnel_cli",
]
resolver = "2"
```

CLI crate 依赖本地 crate：

```toml
[dependencies]
ferris_tunnel_core = { path = "../ferris_tunnel_core" }
ferris_tunnel_net = { path = "../ferris_tunnel_net" }
```

关键命令：

```bash
cargo new crates/ferris_tunnel_core --lib
cargo new crates/ferris_tunnel_net --lib
cargo new crates/ferris_tunnel_cli --bin
cargo run -p ferris_tunnel_cli
```

### 关键 API 参考

- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)
- [Managing Growing Projects with Packages, Crates, and Modules](https://doc.rust-lang.org/book/ch07-00-managing-growing-projects-with-packages-crates-and-modules.html)

---

## 5. 延伸阅读推荐

1. [The Rust Programming Language: Getting Started](https://doc.rust-lang.org/book/ch01-00-getting-started.html)  
   熟悉 `rustc`、`cargo`、`cargo run` 和基础开发流程。

2. [The Rust Book: Packages and Crates](https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html)  
   理解 package、crate、module 的基础关系。

3. [Cargo Reference: Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)  
   Day1 最重要的工程文档。重点看 workspace members、resolver、共享 lockfile。

4. [Rust API Guidelines: Organization](https://rust-lang.github.io/api-guidelines/organization.html)  
   学会如何组织 crate API。系统网络项目后期会非常依赖这套设计习惯。
