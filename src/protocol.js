export const PRODUCT_IDS = [4640, 4641, 4651, 4695, 4696, 4921, 4917, 4888];
export const FILTERS = PRODUCT_IDS.map(productId => ({ vendorId: 0x373b, productId, usagePage: 0xff60, usage: 0x61 }));
export function matchesDevice(device) {
  return device.vendorId === 0x373b && PRODUCT_IDS.includes(device.productId) && device.collections.some(c => c.usagePage === 0xff60 && c.usage === 0x61);
}
// These packets follow the observed ATK HUB 3.2.32 RGB stream, not firmware/storage commands.
export function versionRequest() { const p = new Uint8Array(63); p.set([2, 150]); return p; }
export function parseVersion(data) {
  const p = new Uint8Array(data.buffer, data.byteOffset, data.byteLength);
  // ATK's channel-60 reader checks response command 2 and version bytes 5/6.
  if (p.length < 8 || p[0] !== 2) return null;
  const major = p[5].toString(16), minor = p[6].toString(16).padStart(2, '0');
  if (!/^\d+$/.test(major + minor)) return null;
  const code = Number(major + minor);
  return code >= 100 && code < 1000 ? { code, label: `${major}.${minor}` } : null;
}
export function modePacket() { const p = new Uint8Array(63); p.set([7, 65, 46, 0, 127, 255, 0, 255, 100]); return p; }
export function buildPackets(frame, matrix, versionCode, maxPayload) {
  const keys = matrix.flat();
  if (frame.length !== keys.length || !Number.isInteger(versionCode) || versionCode < 100) throw new Error('LED matrisi veya firmware bilgisi geçersiz.');
  const limit = versionCode >= 200 ? 112 : 41;
  const packets = [];
  for (let i = 0; i < frame.length; i += limit) {
    const colors = frame.slice(i, i + limit);
    const packet = new Uint8Array(4 + colors.length * 3);
    packet.set([0x1d, keys[i].y, keys[i].x, colors.length]);
    colors.forEach((rgb, k) => {
      if (!Array.isArray(rgb) || rgb.length !== 3 || rgb.some(v => !Number.isInteger(v) || v < 0 || v > 255)) throw new Error('RGB değeri geçersiz.');
      packet.set(rgb, 4 + k * 3);
    });
    if (packet.length > maxPayload) throw new Error('HID raporu bu RGB paketini taşımıyor; cihaz desteği doğrulanmalı.');
    packets.push(packet);
  }
  return packets;
}
