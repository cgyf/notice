use android_activity::AndroidApp;
use log::LevelFilter;
use std::sync::OnceLock;

// 直接把 UI 定义写在代码里，不需要 build.rs 和额外的 slint 文件
slint::slint! {
    export component AppWindow inherits Window {
        width: 400px;
        height: 300px;
        background: #202020;

        Text {
            text: "Hello Slint!";
            font-size: 60px;
            color: #4CAF50;
            horizontal-alignment: center;
            vertical-alignment: center;
        }
    }
}

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
