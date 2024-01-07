const MS_IN_SECOND = 1000;
const SECONDS_IN_MINUTE = 60;

export const prettyMillis = (millis: number): string => {
  const seconds = Math.floor(millis / MS_IN_SECOND);
  const minutes = Math.floor(seconds / SECONDS_IN_MINUTE);

  if (minutes >= 1) {
    const remSeconds = Math.floor(seconds % SECONDS_IN_MINUTE);
    return `${minutes}m ${remSeconds}s`;
  } else if (seconds >= 1) {
    const remMillis = Math.floor(millis % MS_IN_SECOND);
    return `${seconds}s ${remMillis}ms`;
  } else {
    return `${Math.floor(millis)}ms`;
  }
};
