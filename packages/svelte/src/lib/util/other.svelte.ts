export const isUrl = (url: string) => {
  try {
    let _ = new URL(url);
    return true;
  } catch (_) {
    return false;
  }
};

export const deepCopy = <T>(object: T): T => {
  return JSON.parse(JSON.stringify(object));
};
