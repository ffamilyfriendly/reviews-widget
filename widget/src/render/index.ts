import { SafeAttributes } from "../check_attributes";
import styles from "./index.module.css";
import { get_data, ReturnData } from "../api";

// Widgets
import simplistic from "./widgets/small";
import button from "./widgets/button";
import starrow from "./widgets/star_row";
import reviewcount from "./widgets/review_count";
import reviewcards from "./widgets/review_cards";

export interface I_Renderer {
  render(
    widget_element: HTMLDivElement,
    data: ReturnData,
    attributes: SafeAttributes,
    parent_element: HTMLDivElement
  ): void | Promise<void>;
}

export const renderers: { [index: string]: I_Renderer } = {
  simplistic,
  button,
  starrow,
  reviewcount,
  reviewcards,
};

function create_attribution() {
  const main = document.createElement("small");
  main.innerHTML = `provided by <a href="https://botsuite.co">botsuite</a>. Data from <a href="https://top.gg">top.gg</a>`;
  return main;
}

export default async function render(
  element: HTMLDivElement,
  attributes: SafeAttributes
) {
  element.classList.add(
    styles.manrope500,
    styles[attributes.theme ?? "dark"],
    styles.barebones
  );

  const ref_link = document.createElement("a");
  ref_link.href = `https://top.gg/bot/${attributes.botid}#reviews`;
  ref_link.classList = styles.widgetLink;
  const widget = document.createElement("div");
  widget.style.setProperty("width", attributes.width);
  widget.style.setProperty("height", attributes.height);
  widget.style.setProperty("border-radius", attributes.borderradius);
  widget.classList.add(styles.default);

  ref_link.appendChild(widget);

  const data = await get_data(attributes.botid);

  if (data.ok) {
    element.appendChild(ref_link);

    renderers[attributes.style].render(widget, data.data, attributes, element);

    if (!attributes["hide-attribution"]) {
      element.appendChild(create_attribution());
    }
  } else {
    throw data.data;
  }
}
