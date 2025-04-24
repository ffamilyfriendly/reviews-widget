import { log } from "./logger";
import { check_attributes } from "./check_attributes";
import render from "./render";
import { render_error } from "./render/error";

function render_widgets() {
  const elements = document.querySelectorAll("div.topgg-widget");

  elements.forEach((element) => {
    if (!(element instanceof HTMLDivElement)) {
      // Should be warn
      log("cannot be anything but <div>");
      return;
    }

    const widget_attributes = element
      .getAttributeNames()
      .filter((attr) => attr.startsWith("data-"))
      .reduce(function (map: { [index: string]: string }, attr) {
        let attribute_value = element.getAttribute(attr);

        if (!attribute_value) {
          attribute_value = "";
          // Should be warn
          log("no attribute");
        }

        map[attr.split("data-")[1]] = attribute_value;
        return map;
      }, {});

    let is_ok = check_attributes(widget_attributes);

    // We're making sure the classlist and content is empty in case a render happens after the initial render
    // this is really only used rn for the widget visualiser but ykyk
    element.innerHTML = "";
    element.classList = "topgg-widget";

    if (is_ok.ok) {
      render(element, is_ok.data).catch((e) => {
        render_error(element, e);
      });
    } else {
      render_error(element, is_ok.data as Error);
    }
  });
}

render_widgets();

declare global {
  interface Window {
    topgg: {};
  }
}

window.topgg = {
  render_widgets,
};
