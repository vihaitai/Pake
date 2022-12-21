fn main() -> wry::Result<()> {
    use wry::{
        application::{
            accelerator::{Accelerator, SysMods},
            event::{Event, StartCause, WindowEvent},
            event_loop::{ControlFlow, EventLoop},
            keyboard::KeyCode,
            menu::{MenuBar as Menu, MenuItem, MenuItemAttributes, MenuType},
            platform::macos::WindowBuilderExtMacOS,
            window::{Window, WindowBuilder, Fullscreen},
        },
        webview::WebViewBuilder,
    };

    let mut menu_bar_menu = Menu::new();
    let mut first_menu = Menu::new();

    first_menu.add_native_item(MenuItem::Hide);
    first_menu.add_native_item(MenuItem::EnterFullScreen);
    first_menu.add_native_item(MenuItem::Minimize);
    first_menu.add_native_item(MenuItem::Separator);
    first_menu.add_native_item(MenuItem::Copy);
    first_menu.add_native_item(MenuItem::Cut);
    first_menu.add_native_item(MenuItem::Paste);
    first_menu.add_native_item(MenuItem::Undo);
    first_menu.add_native_item(MenuItem::Redo);
    first_menu.add_native_item(MenuItem::SelectAll);
    first_menu.add_native_item(MenuItem::Separator);
    let close_item = first_menu.add_item(
        MenuItemAttributes::new("CloseWindow")
            .with_accelerators(&Accelerator::new(SysMods::Cmd, KeyCode::KeyW)),
    );
    first_menu.add_native_item(MenuItem::Quit);

    menu_bar_menu.add_submenu("App", true, first_menu);

    let script = r#"
  (function () {
    window.addEventListener('DOMContentLoaded', (event) => {
      const style = document.createElement('style');
      style.innerHTML = `
        ._app {
          padding-top:30px;
        }

        ._header {
          top: 30px;
        }

        ._sidebar {
          top: 30px;
        }

        #pack-top-dom:active {
          cursor: grabbing;
          cursor: -webkit-grabbing;
        }

        #pack-top-dom{
          position:fixed;
          background:rgb(51, 55, 58);
          z-index:100;
          top:0;
          width:100%;
          height:30px;
          cursor: move;
          cursor: grab;
          cursor: -webkit-grab;
        }
      `;
      document.head.append(style);
      const topDom = document.createElement("div");
      topDom.id = "pack-top-dom"
      document.body.appendChild(topDom);

      const domEl = document.getElementById('pack-top-dom');

      domEl.addEventListener('mousedown', (e) => {
        if (e.buttons === 1 && e.detail !== 2) {
          window.ipc.postMessage('drag_window');
        }
      })

      domEl.addEventListener('touchstart', (e) => {
          window.ipc.postMessage('drag_window');
      })

      domEl.addEventListener('dblclick', (e) => {
          window.ipc.postMessage('fullscreen');
      })

      document.addEventListener('keyup', function (event) {
        if (event.key == "ArrowUp" && event.metaKey){
          scrollTo(0,0);
        }
        if (event.key == "ArrowDown" && event.metaKey){
          window.scrollTo(0, document.body.scrollHeight);
        }
        if (event.key == "ArrowLeft" && event.metaKey){
          window.history.go(-1);
        }
        if (event.key == "ArrowRight" && event.metaKey){
          window.history.go(1);
        }
        if (event.key == "r" && event.metaKey){
          window.location.reload();
        }
      })

    });
  })();
  "#;

    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("DevDocs")
        .with_resizable(true)
        .with_titlebar_transparent(true)
        .with_fullsize_content_view(true)
        .with_titlebar_buttons_hidden(false)
        .with_title_hidden(true)
        .with_menu(menu_bar_menu)
        .with_inner_size(wry::application::dpi::LogicalSize::new(1200.00, 728.00))
        .build(&event_loop)
        .unwrap();

    let handler = move |window: &Window, req: String| {
        if req == "drag_window" {
          let _ =  window.drag_window();
        } else if req == "fullscreen" {
          if window.fullscreen().is_some() {
            window.set_fullscreen(None);
          }else{
            window.set_fullscreen(Some(Fullscreen::Borderless(None)));
          }
        }
    };

    let _webview = WebViewBuilder::new(window)?
        .with_url("https://devdocs.io/")?
        .with_devtools(true)
        .with_initialization_script(script)
        .with_ipc_handler(handler)
        .build()?;

    // _webview.open_devtools();
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::NewEvents(StartCause::Init) => println!("Wry has started!"),
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::MenuEvent {
                menu_id,
                origin: MenuType::MenuBar,
                ..
            } => {
                if menu_id == close_item.clone().id() {
                    _webview.window().set_minimized(true);
                }
                println!("Clicked on {:?}", menu_id);
            }
            _ => (),
        }
    });
}
