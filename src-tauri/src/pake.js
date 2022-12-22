/**
 * @typedef {string} KeyboardKey `event.key` 的代号，
 * 见 <https://developer.mozilla.org/en-US/docs/Web/API/KeyboardEvent/key/Key_Values>
 * @typedef {() => void} OnKeyDown 使用者按下 [CtrlKey] 或者 ⌘ [KeyboardKey]时应该执行的行为
 * 以 Ctrl键或者Meta 键 (⌘) 为首的快捷键清单。
 * 每个写在这里的 shortcuts 都会运行 {@link Event.preventDefault}.
 * @type {Record<KeyboardKey, OnKeyDown>}
 */

const metaKeyShortcuts = {
  ArrowUp: () => scrollTo(0, 0),
  ArrowDown: () => scrollTo(0, document.body.scrollHeight),
  "[": () => window.history.back(),
  "]": () => window.history.forward(),
  r: () => window.location.reload(),
  "-": () => zoomOut(),
  "=": () => zoomIn(),
  "+": () => zoomIn(),
  0: () => zoomCommon(() => "100%"),
};

const ctrlKeyShortcuts = {
  ArrowUp: () => scrollTo(0, 0),
  ArrowDown: () => scrollTo(0, document.body.scrollHeight),
  ArrowLeft: () => window.history.back(),
  ArrowRight: () => window.history.forward(),
  r: () => window.location.reload(),
  "-": () => zoomOut(),
  "=": () => zoomIn(),
  "+": () => zoomIn(),
  0: () => zoomCommon(() => "100%"),
};

window.addEventListener("DOMContentLoaded", (_event) => {
  const style = document.createElement("style");
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
  topDom.id = "pack-top-dom";
  document.body.appendChild(topDom);

  const domEl = document.getElementById("pack-top-dom");

  domEl.addEventListener("mousedown", (e) => {
    if (e.buttons === 1 && e.detail !== 2) {
      window.ipc.postMessage("drag_window");
    }
  });

  domEl.addEventListener("touchstart", () => {
    window.ipc.postMessage("drag_window");
  });

  domEl.addEventListener("dblclick", () => {
    window.ipc.postMessage("fullscreen");
  });

  document.addEventListener("keyup", function (event) {
    const preventDefault = (f) => {
      event.preventDefault();
      f();
    };
    if (/windows|linux/i.test(navigator.userAgent)) {
      if (event.ctrlKey && event.key in ctrlKeyShortcuts) {
        preventDefault(ctrlKeyShortcuts[event.key]);
      }
    }
    if (/macintosh|mac os x/i.test(navigator.userAgent)) {
      if (event.metaKey && event.key in metaKeyShortcuts) {
        preventDefault(metaKeyShortcuts[event.key]);
      }
    }
  });

  document.addEventListener("click", (e) => {
    const origin = e.target.closest("a");
    if (origin && origin.href) {
      const target = origin.target
      origin.target = "_self";
      const hrefUrl = new URL(origin.href)

      if (
        window.location.host !== hrefUrl.host && // 如果 a 标签内链接的域名和当前页面的域名不一致 且
        target === '_blank' // a 标签内链接的 target 属性为 _blank 时
      ) {
        e.preventDefault();
        window.ipc.postMessage(`open_browser:${origin.href}`);
      }
    }
  });
});

setDefaultZoom();

function setDefaultZoom() {
  const htmlZoom = window.localStorage.getItem("htmlZoom");
  if (htmlZoom) {
    document.getElementsByTagName("html")[0].style.zoom = htmlZoom;
  }
}

/**
 * @param {(htmlZoom: string) => string} [zoomRule]
 */
function zoomCommon(zoomRule) {
  const htmlZoom = window.localStorage.getItem("htmlZoom") || "100%";
  const html = document.getElementsByTagName("html")[0];
  const zoom = zoomRule(htmlZoom);
  html.style.zoom = zoom;
  window.localStorage.setItem("htmlZoom", zoom);
}

function zoomIn() {
  zoomCommon((htmlZoom) => `${Math.min(parseInt(htmlZoom) + 10, 200)}%`);
}

function zoomOut() {
  zoomCommon((htmlZoom) => `${Math.max(parseInt(htmlZoom) - 10, 30)}%`);
}
