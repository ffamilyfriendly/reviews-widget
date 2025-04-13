import styling from "./star.module.css";

function create_path() {
  const path = document.createElementNS("http://www.w3.org/2000/svg", "path");
  path.setAttribute("stroke-width", "2");
  path.setAttribute(
    "d",
    "M8.10314 1.05616L6.06251 5.19366L1.49689 5.85928C0.67814 5.97803 0.350015 6.98741 0.943765 7.56554L4.24689 10.7843L3.46564 15.3312C3.32501 16.153 4.19064 16.7687 4.91564 16.3843L9.00002 14.2374L13.0844 16.3843C13.8094 16.7655 14.675 16.153 14.5344 15.3312L13.7531 10.7843L17.0563 7.56554C17.65 6.98741 17.3219 5.97803 16.5031 5.85928L11.9375 5.19366L9.89689 1.05616C9.53127 0.318658 8.47189 0.309283 8.10314 1.05616Z"
  );
  return path;
}

interface I_StarProps {
  fill: "filled" | "unfilled";
  height?: number;
  width?: number;
}

export function create_star({ fill, height = 17, width = 20 }: I_StarProps) {
  const svg = document.createElementNS("http://www.w3.org/2000/svg", "svg");
  svg.classList.add(styling[fill]);
  svg.setAttribute("fill", "none");
  svg.setAttribute("width", width.toString());
  svg.setAttribute("height", height.toString());
  svg.setAttribute("viewBox", "0 0 18 17");
  svg.appendChild(create_path());
  return svg;
}

interface I_StarRowProps {
  amount?: number;
  amount_filled?: number;
  height?: number;
  width?: number;
}

export function create_star_row({
  amount = 5,
  height,
  width,
  amount_filled = amount,
}: I_StarRowProps) {
  const row = document.createElement("div");
  row.classList.add(styling.row);

  for (let i = 0; i < amount; i++) {
    const fill_mode = amount_filled > i ? "filled" : "unfilled";
    const span = document.createElement("span");
    span.className = styling.span;
    const star = create_star({ fill: fill_mode, height, width });
    span.appendChild(star);
    row.appendChild(span);
  }

  return row;
}
