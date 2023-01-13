# atosl-node

项目使用 [create-neon](https://www.npmjs.com/package/create-neon) 构建

具体实现使用了 [atosl-rs](https://github.com/everettjf/atosl-rs)

该库仅仅是将 atosl-rs 变为可直接node调用的依赖

# 关于符号化

[iOS 崩溃日志在线符号化](https://juejin.cn/post/7024000881532076063)

## 安装

必要环境配置 [NodeJs && Rust](https://github.com/neon-bindings/neon#platform-support).

初始化:

```sh
$ npm install
```

## 编译 atosl-node

```sh
$ npm run build
```

该命令使用  [cargo-cp-artifact](https://github.com/neon-bindings/cargo-cp-artifact) 把Rust代码打包成 `./index.node`.

## 使用

```
const atosl = require('./index.node');
// type atosl.parse = (
  option: {
    file: string              //文件完整路径 ( dylib || dwarf )
    load_address: string      //起始地址
    addresses: Array<string>  //运行地址
  },
  text_segment?: boolean      // 默认false
) => {
  success: boolean, 
  data: Array<{address: string, result: string}>
  message?: string
}
const data = atosl.parse({
    file: "/Users/packy/Desktop/TestAapp.dSYM/Contents/Resources/DWARF/Flutter",
    load_address: '0x109810000',
    addresses: [
        '0x0000000109ad88b0',
    ],
});
console.log(data);
/**
成功: 
{
  success: true,
  data: [
    {
      address: 4457334960,
      result: 'GrMtlCommandBuffer::getRenderCommandEncoder(MTLRenderPassDescriptor*, GrMtlPipelineState const*, GrMtlOpsRenderPass*) (in Flutter) + 408'
    },
  ],
  message: null,
}
失败: 
{
  success: false,
  data: [],
  message: 'Unsupported file format'
}
**/
```

## 可用命令

项目根目录下:

### `npm install`

安装项目所需依赖

### `npm build` 和 `npm build-debug`

从Rust源码构建Node依赖 (`index.node`)

Rust构建参数[`cargo build`](https://doc.rust-lang.org/cargo/commands/cargo-build.html)需要通过  `npm build` 和 `npm build-*` 命令执行. 例如: [cargo feature](https://doc.rust-lang.org/cargo/reference/features.html):

```
npm run build -- --feature=beetle
```

#### `npm build-release`

和 [`npm build`](#npm-build)等同, 但是执行cargo release(https://doc.rust-lang.org/cargo/reference/profiles.html#release)
## 项目目录

```
atosl-node/
├── Cargo.toml
├── README.md
├── index.node
├── package.json
├── src/
    ├── atosl.rs
    ├── demangle.rs
|   └── lib.rs
└── target/
```

### Cargo.toml

Rust项目的[配置文件](https://doc.rust-lang.org/cargo/reference/manifest.html)

### index.node

构建产物

### package.json

npm的[配置文件](https://docs.npmjs.com/cli/v7/configuring-npm/package-json)

### src/

Rust源码目录

### src/atosl.rs

atosl主要调用方法

### src/demangle.rs

atosl util

### src/lib.rs

Rust项目入口文件

### target/

二进制Rust产物

## 相关文档

[Neon documentation](https://neon-bindings.com).

[Rust documentation](https://www.rust-lang.org).

[Node documentation](https://nodejs.org).
