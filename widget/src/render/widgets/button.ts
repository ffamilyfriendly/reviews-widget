import { I_Renderer } from "..";
import { ReturnData } from "../../api";
import { SafeAttributes } from "../../check_attributes";
import { create_image } from "../components/image";
import { create_star_row } from "../components/star";
import { create_watermark } from "../components/watermark";
import styling from "./button.module.css";
import common from "../index.module.css";
import { display_count } from "../utils";

const renderer: I_Renderer = {
  render: async function (
    element: HTMLDivElement,
    data: ReturnData,
    attributes: SafeAttributes,
    parent_element
  ): Promise<void> {
    element.classList.add(styling.widget);
    parent_element.classList.add(styling.parent);

    const image_container = document.createElement("div");
    image_container.classList = styling.imageContainer;

    // force dark logo on this one
    const logo = create_watermark("dark");
    image_container.append(logo);

    const text = document.createElement("p");

    text.innerText = `${display_count(data.review_stats.review_count)} reviews`;

    element.append(image_container, text);
    return;
  },
};

export default renderer;
