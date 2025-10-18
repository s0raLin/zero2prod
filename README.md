# Rust Web 应用 (Zero to Production)

这是一个基于 Rust 语言构建的 Web 应用程序，旨在展示如何使用 Rust 构建生产级 Web 服务。该项目是学习《Zero To Production In Rust》一书的实践项目。

## 功能特性

- 健康检查端点 (Health Check)
- 订阅服务 (Subscriptions)
- 数据库集成 (PostgreSQL)
- 配置管理
- 自动化测试

## 技术栈

- **语言**: Rust
- **Web 框架**: actix-web
- **异步运行时**: tokio
- **数据库**: PostgreSQL (通过 sqlx)
- **配置管理**: config + serde
- **测试**: tokio-test, reqwest

## 项目结构

```
hello/
├── src/
│   ├── main.rs          # 应用程序入口
│   ├── lib.rs           # 应用程序核心逻辑
│   ├── configuration.rs  # 配置管理
│   └── routes/          # 路由处理
│       ├── mod.rs
│       ├── health_check.rs
│       └── subscriptions.rs
├── tests/               # 集成测试
│   ├── health_check.rs
│   ├── configuration.yaml
│   └── migrations/      # 数据库迁移文件
└── Cargo.toml           # 项目依赖配置
```

## API 端点

### 健康检查
```
GET /health_check
```
返回 200 OK 状态，用于检查服务是否正常运行。

### 订阅服务
```
POST /subscriptions
Content-Type: application/x-www-form-urlencoded

name={name}&email={email}
```
用于创建新的订阅者记录。

## 配置

项目使用 `configuration.yaml` 文件进行配置：

```yaml
application_port: 8080
database:
  host: "127.0.0.1"
  port: 5432
  username: "postgres"
  password: "password"
  database_name: "newsletter"
```

## 数据库设置

项目使用 PostgreSQL 数据库，并通过 sqlx 进行数据库操作。需要确保：

1. PostgreSQL 服务正在运行
2. 配置文件中的数据库连接信息正确
3. 数据库用户具有创建数据库和表的权限

数据库表结构通过迁移文件定义，会自动创建 subscriptions 表：

```sql
create table subscriptions( 
    id uuid not null,  
    primary key (id), 
    email text not null unique,
    name text not null,
    subscriptions_at timestamptz not null
)
```

## 运行项目

### 环境要求

- Rust 工具链 (推荐使用最新稳定版)
- PostgreSQL 数据库

### 安装和运行

1. 克隆项目到本地：
```bash
git clone <项目地址>
cd RustWeb
```

2. 配置数据库连接信息：
编辑 `hello/configuration.yaml` 文件，设置正确的数据库连接参数。

3. 运行应用程序：
```bash
cargo run --package hello
```

应用程序将在 `http://127.0.0.1:8080` 上启动。

## 测试

项目包含集成测试，可以通过以下命令运行：

```bash
cargo test --package hello
```

测试会自动创建临时数据库并运行迁移脚本，确保测试环境的独立性。

## 开发

### 代码格式化

使用 Rust 标准代码格式化工具：

```bash
cargo fmt
```

### 代码检查

使用 Clippy 进行代码质量检查：

```bash
cargo clippy
```

## 许可证

本项目仅供学习和参考使用。