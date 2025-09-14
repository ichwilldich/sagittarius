export const arrayBufferToBase64 = (buffer: ArrayBuffer) => {
  let byteArray = new Uint8Array(buffer);

  let string = '';
  byteArray.forEach((b) => {
    string += String.fromCharCode(b);
  });

  return btoa(string);
};

export const base64ToArrayBuffer = (base64: string) => {
  let string = atob(base64);
  let len = string.length;

  let buffer = new ArrayBuffer(len);
  let bytes = new Uint8Array(buffer);

  for (let i = 0; i < len; i++) {
    bytes[i] = string.charCodeAt(i);
  }

  return buffer;
};
