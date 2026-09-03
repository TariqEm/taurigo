import { THEME_STORAGE_KEY } from "@taurigo/ui/theme/constants";

/**
 * Inline, blocking script source — inject via a plain `<script>` tag in
 * index.html's `<head>`, before the React bundle loads, so the `.dark` class
 * is applied before first paint (no next-themes-style flash of the wrong
 * theme). Kept as a string rather than a React effect because effects only
 * run after the DOM has already painted once.
 */
export function themeNoFlashScript(
  storageKey: string = THEME_STORAGE_KEY,
): string {
  return `(function(){try{var t=localStorage.getItem(${JSON.stringify(storageKey)});var d=t==="dark"||(t!=="light"&&window.matchMedia("(prefers-color-scheme: dark)").matches);document.documentElement.classList.toggle("dark",d);}catch(e){}})();`;
}
