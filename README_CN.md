# rust-wasm

> 你可以在这里看英文版文档 [English](README.md)

这是一个简单的示例，展示了如何使用 Rust 和 WebAssembly 一起构建 Web 应用程序。

## 安装

你需要安装 Rust 开发工具。你可以通过运行以下命令来完成：

```sh
curl https://sh.rustup.rs -sSf | sh
```

## 运行数据库

```sh
cd db
docker-compose up -d
```

现在你通过 `localhost:5432` 上访问数据库，用户名为 `postgres`，密码为 `postgres`。

你还可以在浏览器上访问 `localhost:8081`，用户名为 `postgres`，密码为 `postgres`。

## 运行 webservice 和 webapp

首先，你应该设置 `DATABASE_URL` 环境变量，包含数据库凭据，如下所示：`postgres://postgres:postgres@localhost:5432/postgres`
，放在项目根目录的 .env 文件中。

其次，你应该设置 `HOST_PORT` 环境变量，包含主机机器的 IP 地址和端口，如下所示：`http://localhost:8080`，放在 webapp 根目录的
.env 文件中。

要运行 webservice，你可以运行：

```sh
cd webservice
cargo run
```

现在你可以打开另一个终端并运行 webapp。要运行 webapp，你可以运行：

```sh
cd webapp
cargo run
```

## 运行 WebAssembly

要运行 WebAssembly，首先你需要安装 `wasm-pack` 工具：

```sh
cargo install wasm-pack
```

然后，你可以运行以下命令：

```sh
cd wasm-client
wasm-pack build
```

安装 npm 依赖：

```sh
cd wasm-client/www
npm install
```

运行 WebAssembly 应用：

```sh
npm run start
```

## 构建

构建webservices和webapp：

```sh
cargo build --bin webservice
cargo build --bin webapp
```

构建 WebAssembly 应用：

```sh
cd wasm-pack
wasm-pack build --release
cd www
npm run build
```
