fn main() {
    // 把 ui/ 下的 .slint 编译成 Rust 代码，src/lib.rs 里用 slint::include_modules!() 引入。
    //
    // 这里显式点出文件名而不是扫描整个目录：只有被列出来的 UI 参与编译，
    // 顺带让 cargo 知道改动哪些文件需要重新构建（slint-build 会输出 rerun-if-changed）。
    // 入口文件 import 进来的 theme.slint / types.slint / components/*.slint 属于同一次编译，
    // slint-build 会给它们一并输出 rerun-if-changed，所以改任何组件都会触发重新构建。
    slint_build::compile("ui/app-window.slint").expect("编译 Slint UI 失败");
}
