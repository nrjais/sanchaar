const MS_IN_SECOND = 1000;
const SECONDS_IN_MINUTE = 60;
const MINUTES_IN_HOUR = 60;

export const prettyMillis = (millis: number): string => {
  const seconds = millis / MS_IN_SECOND;
  const minutes = seconds / SECONDS_IN_MINUTE;
  const hours = minutes / MINUTES_IN_HOUR;

  if (hours >= 1) {
    return `${Math.floor(hours)}h ${Math.floor(minutes % MINUTES_IN_HOUR)}m`;
  } else if (minutes >= 1) {
    return `${Math.floor(minutes)}m ${Math.floor(
      seconds % SECONDS_IN_MINUTE
    )}s`;
  } else if (seconds >= 1) {
    return `${Math.floor(seconds)}s ${Math.floor(millis % MS_IN_SECOND)}ms`;
  } else {
    return `${Math.floor(millis)}ms`;
  }
};
