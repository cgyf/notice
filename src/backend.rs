// 通知数据的来源。
//
// 界面上要显示的内容（通知列表、卡片标签文案、置顶公告、顶部日期）全部在这里组装，
// ui/ 下的 .slint 文件只负责渲染，不写死任何文案。
//
// 现在用的是示例数据；换成真实接口时只改这个文件（比如在这里发请求、拿到 JSON 后
// 转成下面这些结构），UI 侧代码不用动。

use slint::{ModelRc, SharedString, VecModel};

use crate::{Notice, NoticeChip, PinnedNotice};

// 标签色调：0=品牌蓝 1=紫 2=绿 3=橙(警示)。
// 后端只给语义化的色调，具体色值由前端主题决定，后端不需要知道主题长什么样。
const TONE_ACCENT: i32 = 0;
const TONE_VIOLET: i32 = 1;
const TONE_OK: i32 = 2;
const TONE_WARN: i32 = 3;

// 把 (文案, 色调) 列表转成界面要的模型
fn chips(items: &[(&str, i32)]) -> ModelRc<NoticeChip> {
    ModelRc::new(VecModel::from(
        items
            .iter()
            .map(|(label, tone)| NoticeChip { label: SharedString::from(*label), tone: *tone })
            .collect::<Vec<_>>(),
    ))
}

// 通知列表
pub fn notices() -> ModelRc<Notice> {
    ModelRc::new(VecModel::from(vec![
        Notice {
            kind: 0, // 公告
            title: SharedString::from("Slint 1.18 已发布"),
            body: SharedString::from("新的渲染后端上线，启动更快、动画更顺滑，建议升级。"),
            chips: chips(&[("公告", TONE_ACCENT), ("重要", TONE_WARN)]),
            time: SharedString::from("09:24"),
            unread: true,
            important: true,
        },
        Notice {
            kind: 1, // 活动
            title: SharedString::from("本周五线上分享会"),
            body: SharedString::from("主题：用一个下午写个 Android 小程序。报名截止周四 18:00。"),
            chips: chips(&[("活动", TONE_VIOLET)]),
            time: SharedString::from("昨天"),
            unread: true,
            important: false,
        },
        Notice {
            kind: 2, // 安全
            title: SharedString::from("新的登录设备"),
            body: SharedString::from("Pixel 8 · 上海 · 刚刚。如果不是你本人操作，请立刻修改密码。"),
            chips: chips(&[("安全", TONE_OK)]),
            time: SharedString::from("昨天"),
            unread: true,
            important: false,
        },
        Notice {
            kind: 3, // 更新
            title: SharedString::from("构建流水线已修复"),
            body: SharedString::from("缓存策略调整完成，CI 构建时间从 6 分钟降到 3 分钟。"),
            chips: chips(&[("更新", TONE_WARN)]),
            time: SharedString::from("周二"),
            unread: false,
            important: false,
        },
        Notice {
            kind: 1, // 活动
            title: SharedString::from("test notice entry"),
            body: SharedString::from("这是我的第一个测试notice entry。"),
            chips: chips(&[("活动", TONE_VIOLET)]),
            time: SharedString::from("现在"),
            unread: false,
            important: false,
        },
    ]))
}

// 顶部置顶公告条
pub fn pinned() -> PinnedNotice {
    PinnedNotice {
        label: SharedString::from("置顶 · 公告"),
        title: SharedString::from("Slint 1.18 已发布"),
        summary: SharedString::from("新的渲染后端，启动更快、动画更顺滑。"),
    }
}

// 顶部日期（真实项目里按当前时区算出来）
pub fn today() -> SharedString {
    SharedString::from("2026 年 9 月 24 日 · 星期四")
}
