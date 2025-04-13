import { I_Renderer } from "..";
import { ReturnData } from "../../api";
import { SafeAttributes } from "../../check_attributes";
import { create_image } from "../components/image";
import { create_star_row } from "../components/star";
import { create_watermark } from "../components/watermark";
import styling from "./small.module.css";
import common from "../index.module.css";
import { into_5_distribution } from "../utils";

const renderer: I_Renderer = {
  render: async function (
    element: HTMLDivElement,
    data: ReturnData,
    attributes: SafeAttributes
  ): Promise<void> {
    element.classList.add(styling.widget);
    const image_container = document.createElement("div");
    image_container.classList = styling.imageContainer;
    image_container.append(
      create_image({
        dimensions: { height: "3em", width: "3em" },
        src: data.icon_url,
      })
    );

    const column = document.createElement("div");
    column.className = styling.column;
    const bot_name = document.createElement("h2");
    bot_name.innerText = data.name;

    const fixed_score = into_5_distribution(data.review_stats.average_score);

    const star_row = create_star_row({
      amount: 5,
      amount_filled: fixed_score,
    });
    const watermark = create_watermark(attributes.theme);

    const score_breakdown = document.createElement("small");
    score_breakdown.classList.add(styling.scoreBreakdown, common.manrope400);
    score_breakdown.innerText = `rated ${fixed_score.toFixed(1)} out of 5 stars`;

    column.append(bot_name, star_row, score_breakdown);

    element.append(image_container, column, watermark);
    return;
  },
};

export default renderer;
