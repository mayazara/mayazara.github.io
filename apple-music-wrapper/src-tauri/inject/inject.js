// Runs in the music.apple.com page before any of its own scripts.
// This is where personal tweaks go. `window.__WRAPPER_CSS__` holds inject.css.
(() => {
  if (window.__WRAPPER_LOADED__) return;
  window.__WRAPPER_LOADED__ = true;

  // Inject our stylesheet as soon as there is a <head> (or <html>) to put it in.
  const addStyle = () => {
    if (document.getElementById("wrapper-css")) return;
    const style = document.createElement("style");
    style.id = "wrapper-css";
    style.textContent = window.__WRAPPER_CSS__ || "";
    (document.head || document.documentElement).appendChild(style);
  };
  if (document.documentElement) {
    addStyle();
  } else {
    document.addEventListener("DOMContentLoaded", addStyle, { once: true });
  }

  // Keyboard shortcuts. Ignored while typing in a text field.
  document.addEventListener("keydown", (e) => {
    const t = e.target;
    if (t && (t.isContentEditable || /^(INPUT|TEXTAREA|SELECT)$/.test(t.tagName))) return;

    // Ctrl+L: jump to the search box.
    if (e.ctrlKey && e.key.toLowerCase() === "l") {
      const search = document.querySelector('input[type="search"]');
      if (search) {
        e.preventDefault();
        search.focus();
      }
    }

    // Alt+Left / Alt+Right: back / forward.
    if (e.altKey && e.key === "ArrowLeft") history.back();
    if (e.altKey && e.key === "ArrowRight") history.forward();
  });
})();
