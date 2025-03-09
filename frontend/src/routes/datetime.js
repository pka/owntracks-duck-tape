export function addDays(date, days) {
  const newDate = new Date(date);
  newDate.setDate(date.getDate() + days);
  return newDate;
}

export function utcToLocalTime(utcTimeString) {
  // input example: '2025-03-01 08:21:16+00'
  const date = new Date(utcTimeString);
  return date.toLocaleTimeString("de-CH");
}

export function utcToLocalDate(utcTimeString) {
  // input example: '2025-03-01 08:21:16+00'
  const date = new Date(utcTimeString);
  return date.toLocaleDateString("de-CH");
}
