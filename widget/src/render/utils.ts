export function into_5_distribution(n: number) {
  return n / 20;
}

export function display_count(n: number) {
  let count = n;
  let count_text = `${count}`;

  const as_thousands = count / 1000;
  if (as_thousands > 1) count_text = `${as_thousands.toFixed(1)}k`;

  return count_text;
}

export function display_number(n: number) {
  const fmt = new Intl.NumberFormat();
  return fmt.format(n);
}

export function relative_timestamp(ts: Date): string {
  const rtf = new Intl.RelativeTimeFormat(undefined, {
    localeMatcher: "best fit",
    numeric: "auto",
    style: "long",
  });

  const delta = ts.getTime() - Date.now();

  const ms_per_minute = 1000 * 60;
  const ms_per_hour = ms_per_minute * 60;
  const ms_per_day = ms_per_hour * 24;
  const ms_per_week = ms_per_day * 7;
  const ms_per_month = ms_per_day * 30;
  const ms_per_year = ms_per_day * 365;

  const absDelta = Math.abs(delta);

  if (absDelta >= ms_per_year) {
    return rtf.format(Math.round(delta / ms_per_year), "year");
  } else if (absDelta >= ms_per_month) {
    return rtf.format(Math.round(delta / ms_per_month), "month");
  } else if (absDelta >= ms_per_week) {
    return rtf.format(Math.round(delta / ms_per_week), "week");
  } else if (absDelta >= ms_per_day) {
    return rtf.format(Math.round(delta / ms_per_day), "day");
  } else if (absDelta >= ms_per_hour) {
    return rtf.format(Math.round(delta / ms_per_hour), "hour");
  } else {
    return rtf.format(Math.round(delta / ms_per_minute), "minute");
  }
}
