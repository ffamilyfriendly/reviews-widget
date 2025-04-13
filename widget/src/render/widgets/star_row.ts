import { I_Renderer } from "..";
import { ReturnData } from "../../api";
import { SafeAttributes } from "../../check_attributes";
import { create_image } from "../components/image";
import { create_star_row } from "../components/star";
import { create_watermark } from "../components/watermark";
import { display_count, into_5_distribution } from "../utils";
import styling from "./star_row.module.css";

const renderer: I_Renderer = {
  render: async function (
    element: HTMLDivElement,
    data: ReturnData,
    attributes: SafeAttributes,
    parent_element
  ): Promise<void> {
    element.classList.add(styling.widget);
    parent_element.classList.add(styling.parent);

    const star_container = document.createElement("div");
    star_container.classList = styling.imageContainer;

    const stars = create_star_row({
      amount: 5,
      amount_filled: into_5_distribution(data.review_stats.average_score),
    });
    star_container.append(stars);

    const text = document.createElement("p");

    text.innerText = `${display_count(data.review_stats.review_count)} reviews`;

    element.append(star_container, text);
    return;
  },
};

export default renderer;
