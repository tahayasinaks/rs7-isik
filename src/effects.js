export const EFFECTS = { reactive: 'Tuşa basınca dalga', wave: 'Sürekli dalga', diagonal: 'Çapraz dalga', rainbow: 'Gökkuşağı', aurora: 'Aurora', fire: 'Ateş', rain: 'Yağmur', stars: 'Yıldızlar', breathing: 'Nefes', solid: 'Sabit renk', off: 'Kapalı' };
function hsv(h, s = 1, v = 1) {
  h = ((h % 1) + 1) % 1;
  const f = n => { const k = (n + h * 6) % 6; return Math.round(255 * v * (1 - s * Math.max(0, Math.min(k, 4 - k, 1)))); };
  return [f(5), f(3), f(1)];
}
const hash = n => { const x = Math.sin(n * 127.1) * 43758.5453; return x - Math.floor(x); };
export function renderFrame(matrix, effect, time, brightness, hex = '#39c5ff', ripples = []) {
  if (!(effect in EFFECTS)) throw new Error('Bilinmeyen efekt.');
  const level = Math.max(0, Math.min(100, Number(brightness) || 0)) / 100;
  const color = /^#[0-9a-f]{6}$/i.test(hex) ? hex.match(/\w\w/g).map(x => parseInt(x, 16)) : [57, 197, 255];
  return matrix.flat().map(({ x, y }) => {
    let rgb;
    switch (effect) {
      case 'reactive': {
        rgb = [0, 0, 0];
        for (const press of ripples) {
          const age = time - press.time;
          if (age < 0 || age >= 2.2) continue;
          const distance = Math.hypot(x - press.x, (y - press.y) * 1.25);
          const intensity = Math.max(0, 1 - Math.abs(distance - age * 8) / 1.5) * (1 - age / 2.2);
          const ring = hsv(distance / 20 - age * .3, .85, intensity);
          rgb = rgb.map((v, i) => Math.max(v, ring[i]));
        }
        break;
      }
      case 'wave': rgb = hsv(x / 18 - time * .14); break;
      case 'diagonal': rgb = hsv((x + y) / 18 - time * .14); break;
      case 'rainbow': rgb = hsv(time * .1); break;
      case 'aurora': rgb = hsv(.42 + .18 * Math.sin(x * .4 + time + y * .5), .8, .4 + .6 * (Math.sin(x * .5 - time) + 1) / 2); break;
      case 'fire': rgb = hsv(.02 + .09 * hash(x + Math.floor(time * 8) * 15 + y), 1, Math.max(.05, 1 - y / 9) * (.5 + .5 * hash(x * 7 + y + Math.floor(time * 8)))); break;
      case 'rain': { const head = (time * (2 + hash(x)) + hash(x) * 8) % 9; rgb = hsv(.55, .8, Math.max(0, 1 - Math.abs(head - y) / 2)); break; }
      case 'stars': rgb = hsv(hash(x + y * 15), .3, Math.max(0, Math.sin(time * 1.7 + hash(x + y * 15) * 40)) ** 12); break;
      case 'breathing': rgb = color.map(v => v * (.5 - .5 * Math.cos(time * 2))); break;
      case 'solid': rgb = color; break;
      default: rgb = [0, 0, 0];
    }
    return rgb.map(v => Math.round(v * level));
  });
}
