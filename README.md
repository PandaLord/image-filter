# Image Filter
一个用来给图片添加滤镜，进行图片处理的小工具

### Roadmap
- 封装Photon-rs的功能，实现图片的处理
  - 添加text功能
  - 
- 实现一个前端，可以用来选择图片，选择处理方式等
  - 可以是一个Tui/Cli
  - 也可以是一个以webview2为基础的桌面应用，例如tauri
  - 也可以是WASM
  - 或者还可以是native app
- 实现一个Http server，让用户可以通过call url的形式对图片进行相关的处理
  - 需要选型http server框架，例如Axum
  - 需要选型url serialize框架，例如profobuf
