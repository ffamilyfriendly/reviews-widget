export const URL_BASE = "https://widget.botsuite.co";

export const ENDPOINTS = {
  GET_BASE_DATA: (bot_id: string) => `${URL_BASE}/widget/${bot_id}` as const,
};

export enum LOG_LEVEL {
  LOW,
  MEDIUM,
  HIGH,
}

export const THEME_COLOURS = [
  "#ff3366", // red
  "#8957ff", // purple
  "#00bbff", // blue
  "#00cc88", // green
  "#ffbb00", // yellow
  "#ff6b00", // orange
];

export const CURRENT_LOG_LEVEL = LOG_LEVEL.LOW;

export const SUPPORT_SERVER = "https://botsuite.co/join";

export const VERSION = { major: 1, minor: 0, patch: 0 };
