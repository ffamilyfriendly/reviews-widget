import styling from "./image.module.css";

type ImageUnit = `${number}px` | `${number}em` | `${number}rem` | `${number}%`;

type ImageDimensions =
  | { width: ImageUnit; height: ImageUnit; aspect_ratio?: never }
  | ({ aspect_ratio: `${number}/${number}` } & (
      | { width: ImageUnit; height?: never }
      | { height: ImageUnit; width?: never }
    ));

interface I_ImageProps {
  class_list?: string[];
  dimensions: ImageDimensions;
  src: string;
}

export function create_image({
  class_list = [],
  src,
  dimensions,
  ...props
}: I_ImageProps) {
  const { height, width, aspect_ratio } = dimensions;

  const img = document.createElement("img");

  img.style.height = height;
  img.style.width = width;
  img.style.aspectRatio = aspect_ratio;

  img.src = src;
  img.classList.add(styling.image, ...class_list);

  img.onerror = (ev) => {
    img.src = "https://top.gg/favicon.png";
    console.log("COULD NOT LOAD IMAGE");
  };

  return img;
}
