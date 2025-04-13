import { I_Renderer } from "..";
import { ReturnData, Review, ScoreDistribution } from "../../api";
import { SafeAttributes } from "../../check_attributes";
import { create_image } from "../components/image";
import { create_star_row } from "../components/star";
import { create_watermark } from "../components/watermark";
import {
  display_count,
  display_number,
  into_5_distribution,
  relative_timestamp,
} from "../utils";
import styling from "./review_cards.module.css";

function create_card(review: Review) {
  const card = document.createElement("div");
  card.classList = styling.reviewcard;

  const img = create_image({
    src: review.author.avatar_url,
    dimensions: { height: "2em", aspect_ratio: "1/1" },
    class_list: [styling.avatar],
  });

  const top_row = document.createElement("div");
  top_row.classList = styling.reviewcardtop;
  const star_row = create_star_row({
    amount: 5,
    amount_filled: review.score / 20,
  });

  const timestamp = document.createElement("time");
  timestamp.dateTime = review.timestamp;
  timestamp.classList = styling.time;
  timestamp.innerText = relative_timestamp(new Date(review.timestamp));

  top_row.append(star_row, timestamp);

  const review_text = document.createElement("p");
  review_text.innerText = review.content;
  review_text.classList = styling.reviewtextcontent;

  const username = document.createElement("p");
  username.innerText = review.author.username;

  const right = document.createElement("div");
  right.classList = styling.reviewtext;
  right.append(username, top_row, review_text);

  card.append(img, right);

  return card;
}

function create_cards(reviews: Review[]) {
  const parent = document.createElement("div");
  parent.classList = styling.reviewsparent;

  for (const review of reviews) {
    parent.appendChild(create_card(review));
  }

  return parent;
}

function create_score_bar(value, max) {
  const outer = document.createElement("div");
  outer.classList = styling.barouter;
  const inner = document.createElement("div");
  inner.classList = styling.barinner;
  const percent_filled = (value / max) * 100;
  inner.style.width = `${percent_filled}%`;

  outer.append(inner);

  return outer;
}

function create_score_breakdown(
  distribution: ScoreDistribution[],
  total_reviews: number
) {
  const container = document.createElement("div");
  container.classList = styling.breakdown;

  for (const score of distribution.reverse()) {
    const star_text = document.createElement("p");
    star_text.innerText = `${score.key / 20} stars`;

    const score_bar = create_score_bar(score.value, total_reviews);

    const reviews_count = document.createElement("p");
    reviews_count.innerText = display_count(score.value);

    container.append(star_text, score_bar, reviews_count);
  }
  return container;
}

function create_aggregate_card(data: ReturnData, attributes: SafeAttributes) {
  const card = document.createElement("div");
  card.classList = styling.aggregate;

  console.log(data.review_stats.score_distribution);

  const star_row = create_star_row({
    amount: 5,
    amount_filled: into_5_distribution(data.review_stats.average_score),
    height: 25,
    width: 25,
  });

  const review_info = document.createElement("p");
  review_info.innerText = "Based on ";
  const review_count = document.createElement("u");
  review_count.innerText = `${display_number(data.review_stats.review_count)} reviews`;
  review_info.appendChild(review_count);
  const watermark = create_watermark(attributes.theme);

  const score_breakdown = create_score_breakdown(
    data.review_stats.score_distribution,
    data.review_stats.review_count
  );

  card.append(star_row, review_info, score_breakdown, watermark);
  return card;
}

const renderer: I_Renderer = {
  render: async function (
    element: HTMLDivElement,
    data: ReturnData,
    attributes: SafeAttributes,
    parent_element
  ): Promise<void> {
    element.classList.add(styling.widget);

    const aggregate = create_aggregate_card(data, attributes);
    const reviews = create_cards(data.reviews);

    element.append(aggregate, reviews);
    return;
  },
};

export default renderer;
