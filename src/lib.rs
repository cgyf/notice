// 整个 crate 只在 Android 上编译：slint / android_logger / android-activity 都声明在
// Cargo.toml 的 cfg(target_os = "android") 依赖里，非 Android 平台下这个 crate 是空的。
#![cfg(target_os = "android")]

use android_activity::AndroidApp;
use log::LevelFilter;
use std::sync::OnceLock;

// 引入 ui/ 下的 UI：由 build.rs 调用 slint-build 编译，生成 AppWindow 等类型。
slint::include_modules!();

static LOGGER_ONCE: OnceLock<()> = OnceLock::new();

#[unsafe(no_mangle)]
fn android_main(app: AndroidApp) {
    LOGGER_ONCE.get_or_init(|| {
        android_logger::init_once(
            android_logger::Config::default()
                .with_max_level(LevelFilter::Info)
                .with_tag("RustApp"),
        );
    });

    log::info!("android_main 启动");

    // 初始化 Slint 的 Android 后端
    slint::android::init(app).unwrap();

    // 创建并运行界面
    AppWindow::new().unwrap().run().unwrap();

    log::info!("Slint 窗口关闭");
}
