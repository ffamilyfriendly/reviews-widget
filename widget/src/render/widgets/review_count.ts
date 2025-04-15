import { I_Renderer } from "..";
import { ReturnData } from "../../api";
import { SafeAttributes } from "../../check_attributes";
import { create_watermark } from "../components/watermark";
import { display_count } from "../utils";
import styling from "./review_count.module.css";

const renderer: I_Renderer = {
  render: async function (
    element: HTMLDivElement,
    data: ReturnData,
    attributes: SafeAttributes,
    parent_element
  ): Promise<void> {
    element.classList.add(styling.widget);

    const see = document.createElement("span");
    see.innerText = "See";
    const review_count = document.createElement("b");

    review_count.innerText = display_count(data.review_stats.review_count);

    const on = document.createElement("span");
    on.innerText = "reviews on";

    const watermark = create_watermark(attributes.theme);
    watermark.classList.add(styling.icon);

    element.append(see, review_count, on, watermark);
    return;
  },
};

export default renderer;
