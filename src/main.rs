use gpui::*;
use gpui_component::*;
use gpui_component::webview::WebView;
use gpui_component::wry;
use raw_window_handle::HasWindowHandle;

pub struct HelloWorld {
    webview: Entity<WebView>,
}

impl Render for HelloWorld {
    fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .child(self.webview.clone())
    }
}

impl Drop for HelloWorld {
    fn drop(&mut self) {
        // 当最后一个窗口的根视图被销毁时，直接退出整个应用进程
        std::process::exit(0);
    }
}

fn main() {
    let app = Application::new();

    app.run(|cx| {
        // 初始化 GPUI Component
        gpui_component::init(cx);

        // 打开窗口并渲染根视图
        cx.open_window(WindowOptions::default(), |window, cx| {
            let view = cx.new(|cx| {
                // 创建 WebView 组件并加载初始页面
                let webview = {
                    let builder = wry::WebViewBuilder::new();
                    let window_handle = window.window_handle().expect("No window handle");
                    let wry_view = builder.build_as_child(&window_handle).unwrap();
                    // 通过 cx.new 包装为 Entity<WebView>
                    let webview = cx.new(|cx| WebView::new(wry_view, window, cx));
                    // 载入初始 URL
                    webview.update(cx, |wv, _| wv.load_url("https://day.xujiong.online/"));
                    webview
                };

                HelloWorld { webview }
            });

            cx.new(|cx| Root::new(view, window, cx))
        })
        .unwrap();
    });
}
