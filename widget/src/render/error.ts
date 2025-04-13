import { log } from "../logger";
import default_style from "./index.module.css";
import styling from "./error.module.css";
import { SUPPORT_SERVER } from "../config";

export function render_error(element: HTMLDivElement, err: Error) {
  element.classList.add(
    default_style.barebones,
    default_style.default,
    default_style.manrope500,
    styling.error
  );
  const error_type = document.createElement("h2");
  error_type.innerText = err.name;
  const error_description = document.createElement("p");
  error_description.innerText = err.message;
  const check_console = document.createElement("small");
  check_console.classList = styling.small;
  check_console.innerText = "check console for more information";

  const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.setAttribute("width", "2em");
  svg.setAttribute("height", "2em");
  svg.setAttribute("viewBox", "0 0 24 24");
  svg.classList = styling.icon;

  svg.innerHTML = `<g fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2"><path stroke-dasharray="64" stroke-dashoffset="64" d="M12 3l9 17h-18l9 -17Z"><animate fill="freeze" attributeName="stroke-dashoffset" dur="0.6s" values="64;0"/></path><path stroke-dasharray="6" stroke-dashoffset="6" d="M12 10v4"><animate fill="freeze" attributeName="stroke-dashoffset" begin="0.6s" dur="0.2s" values="6;0"/></path><path stroke-dasharray="2" stroke-dashoffset="2" d="M12 17v0.01"><animate fill="freeze" attributeName="stroke-dashoffset" begin="0.8s" dur="0.2s" values="2;0"/></path></g>`;

  const text_container = document.createElement("div");
  text_container.append(error_type, error_description, check_console);

  element.append(svg, text_container);

  log(
    `Failed to render top.gg widget. Visit ${SUPPORT_SERVER} for further support.`,
    "Error dump:"
  );
  console.error(err);
}
