import { Err, Ok, Result } from "./api";
import { renderers } from "./render";

const WIDGET_STYLES = Object.keys(renderers) as string[];

interface AttributeValidator {
  name: string;
  required?: boolean;
  matches_regex?: RegExp;
  any_of?: string[];
}

const BORDER_RADIUS_REGEX = /\d+((r|)em|px|%|vh)/i;
type CssUnit =
  | `${number}px`
  | `${number}rem`
  | `${number}em`
  | `${number}%`
  | `${number}vh`;

const REQUIRED_ATTRIBUTES: AttributeValidator[] = [
  {
    name: "botid",
    matches_regex: /\d{17,18}$/i,
    required: true,
  },
  {
    name: "style",
    any_of: WIDGET_STYLES,
    required: true,
  },
  {
    name: "height",
    required: false,
    matches_regex: BORDER_RADIUS_REGEX,
  },
  {
    name: "width",
    required: false,
    matches_regex: BORDER_RADIUS_REGEX,
  },
  {
    name: "theme",
    any_of: ["light", "dark"],
  },
  {
    name: "borderradius",
    required: false,
    matches_regex: BORDER_RADIUS_REGEX,
  },
];

export interface SafeAttributes {
  botid: string;
  style: string;
  height?: CssUnit;
  width?: CssUnit;
  borderradius?: CssUnit;
  theme?: "light" | "dark";
  [index: string]: string;
}

export function check_attributes(attributes: {
  [index: string]: string;
}): Result<SafeAttributes> {
  for (const { name, matches_regex, any_of, required } of REQUIRED_ATTRIBUTES) {
    const value = attributes[name];

    if (!value) {
      if (required)
        return Err(new Error(`missing required attribute '${name}'`));
      else continue;
    }

    if (any_of && !any_of.includes(value)) {
      return Err(
        new Error(
          `attribute '${name}' w/ value '${value}' is not any of '${any_of.join("|")}'`
        )
      );
    }

    if (matches_regex && !matches_regex.test(value)) {
      return Err(
        new Error(
          `attribute '${name}' w/ value '${value}' does not comply with required regex '${matches_regex}'`
        )
      );
    }
  }

  return Ok(attributes as unknown as SafeAttributes);
}
