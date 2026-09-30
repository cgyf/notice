// 整个 crate 只在 Android 上编译：slint / android_logger / android-activity 都声明在
// Cargo.toml 的 cfg(target_os = "android") 依赖里，非 Android 平台下这个 crate 是空的。
#![cfg(target_os = "android")]

use android_activity::AndroidApp;
use log::LevelFilter;
use std::sync::OnceLock;

// 引入 ui/ 下的 UI：由 build.rs 调用 slint-build 编译，生成 AppWindow 等类型。
slint::include_modules!();

// 通知数据的来源（界面要显示的内容都在这里，ui/*.slint 里不写死文案）。
// 以后接真实接口，改这个模块就行。
mod backend;

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

    // 创建界面，并在 run() 之前把数据灌进去：界面上看到的都是后端给的内容
    let ui = AppWindow::new().unwrap();
    ui.set_notices(backend::notices());
    ui.set_pinned(backend::pinned());
    ui.set_today(backend::today());

    ui.run().unwrap();

    log::info!("Slint 窗口关闭");
}
