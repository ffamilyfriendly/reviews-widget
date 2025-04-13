import { CURRENT_LOG_LEVEL, LOG_LEVEL, VERSION } from "./config";

function internal_log(log_level: LOG_LEVEL, ...args: string[]) {
  if (CURRENT_LOG_LEVEL > log_level) return;
  console.log(
    `%c[Top.gg Widget ${VERSION.major}.${VERSION.minor}.${VERSION.patch}] %c${args.join("\n")}`,
    "font-weight: bold; float: left; display: block; height: 100%; font-size: smaller;",
    ""
  );
}

export function log(message: string, ...args: string[]) {
  internal_log(LOG_LEVEL.LOW, message, ...args);
}
