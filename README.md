[在线体验](https://liujiuwu.github.io/game_template/)

# 安装相关工具

## 1 添加wasm target

首先，Rust 需要安装target来处理 Web 程序集 (WASM) 的编译。target名称是`wasm32-unknown-unknown`,使用以下命令来安装它：
```shell
rustup target add wasm32-unknown-unknown

#查看已安装的target
rustup target list --installed
aarch64-apple-darwin
wasm32-unknown-unknown

#查看所有支持的target
rustup target list

```

`wasm32-unknown-unknown` 是 Rust 编程语言中的一个目标平台，用于编译生成 WebAssembly（Wasm）模块。它表示编译后的 Wasm 模块可以在任何支持 WebAssembly 的环境中运行，而不依赖于特定的操作系统或硬件架构。通过使用 `wasm32-unknown-unknown` 目标平台，开发者可以将 Rust 代码编译为高效且可移植的 Wasm 模块，并在浏览器、服务器和其他支持 WebAssembly 的环境中运行这些模块。这种跨平台性使得使用 Rust 开发 WebAssembly 应用程序变得更加方便和灵活。

## 2 安装 wasm-bindgen 工具

wasm-bindgen 是一个用于在 WebAssembly 和 JavaScript 之间进行绑定的工具。它提供了一种简单的方式来调用 WebAssembly 模块中的函数和导出，并将 JavaScript 对象传递给 WebAssembly 模块。使用 wasm-bindgen，您可以编写以 Rust 或其他支持 WebAssembly 的语言编写的模块，并通过生成绑定代码使其与 JavaScript 进行交互。这样，您就可以在浏览器环境中直接使用这些模块，而无需手动处理底层的二进制数据或复杂的内存管理。wasm-bindgen 还提供了一些额外功能，如类型转换、错误处理和高级特性（如迭代器）。它大大简化了在 WebAssembly 和 JavaScript 之间进行通信和交互的过程，使得开发者能够更轻松地利用 WebAssembly 的优势。

```shell
#命令行方式
cargo install wasm-bindgen-cli

```

## 3 操作步骤

### 3.1 步骤一：编译WASM程序
```shell
cargo build --release --target wasm32-unknown-unknown #为了加载速度和性能，最终编译应该是release版本。
```
执行完上面的命令后，后面 target/wasm32-unknown-unknown/release/ 目录下生成一个 xxx.d的调试文件和一个xxx.wasm 目标文件。

### 3.2 步骤二：组装Web文件
```shell
#进入release目录
cd target/wasm32-unknown-unknown/release

#组装
wasm-bindgen xxxx.wasm --out-dir wasm --no-modules --no-typescript
```

执行完上面的命令，将在wasm目录下产生一个xxx.js的文件和一个xxxx_bg.wasm文件。

### 3.3 步骤三：在Web中加载
在wasm目录下创建index.html文件，内容如下：
```html
<!-- 将其中的js和wasm改进你自己的名称 -->
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type" />
  </head>
  <body>
    <canvas id="canvas" width="640" height="480"></canvas>
    <script src="./game.js"></script>
    <script>
      window.addEventListener("load", async () => {
        await wasm_bindgen("./game_bg.wasm");
      });
    </script>
  </body>
</html>
```

### 3.4 步骤四：通过web服务来体验
本地的话，可以安装vs code 的 Live Server 插件。然后在wasm目录执行 “code .” 打开本项目， 点击vs code右下角的 Live Server 将打开浏览器显示wasm结果。
