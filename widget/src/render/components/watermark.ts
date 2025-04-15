import { URL_BASE } from "../../config";
import { create_image } from "./image";
import styling from "./watermark.module.css";

const WATERMARK = {
  DARK: `${URL_BASE}/cdn/topgg_logo.png`,
  LIGHT: `${URL_BASE}/cdn/topgg_logo_black.png`,
};

export function create_watermark(theme: "light" | "dark") {
  const container = document.createElement("div");
  container.classList = styling.container;

  const img = create_image({
    src: theme === "light" ? WATERMARK.LIGHT : WATERMARK.DARK,
    dimensions: {
      height: "20px",
      aspect_ratio: "1369/281",
    },
  });
  img.classList = styling.img;

  container.appendChild(img);
  return container;
}
