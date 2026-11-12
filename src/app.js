import { EFFECTS, renderFrame } from './effects.js';
import { Keyboard } from './device.js';
const matrix = await fetch('src/layout.json').then(r => { if (!r.ok) throw new Error('Klavye matrisi yüklenemedi.'); return r.json(); });
const $ = id => document.getElementById(id);
const keyboard = new Keyboard();
let running = false, busy = false, writing = false, pendingWrite = Promise.resolve(), lastWrite = 0;
const keys = [];
for (const row of matrix) {
  const element = document.createElement('div'); element.className = 'row';
  for (const key of row) { const el = document.createElement('div'); el.className = 'key' + (key.hide ? ' hidden' : ''); el.textContent = key.label || ''; element.append(el); keys.push(el); }
  $('keyboard').append(element);
}
for (const [value, label] of Object.entries(EFFECTS)) { const option = document.createElement('option'); option.value = value; option.textContent = label; $('effect').append(option); }
function state(message) {
  $('status').textContent = message;
  $('connect').disabled = busy || !!keyboard.device;
  $('disconnect').disabled = busy || !keyboard.device;
  $('start').disabled = busy || !keyboard.device || running;
  $('stop').disabled = busy || !running;
}
$('brightness').addEventListener('input', () => $('level').value = `%${$('brightness').value}`);
$('connect').onclick = async () => {
  busy = true; state('Bağlanıyor…');
  let message;
  try { message = await keyboard.connect() ? `Bağlandı · Firmware ${keyboard.version.label} · Başlatılmadı` : 'Cihaz seçilmedi · Önizleme açık'; }
  catch (error) { message = error.message; }
  finally { busy = false; state(message); }
};
$('start').onclick = async () => {
  busy = true; state('Aydınlatma hazırlanıyor…');
  let message;
  try { await keyboard.start(matrix); running = true; message = 'Efekt klavyeye gönderiliyor'; }
  catch (error) { message = error.message; }
  finally { busy = false; state(message); }
};
async function stop(disconnect = false) {
  busy = true; running = false; state('Durduruluyor…');
  let message = 'Durduruldu · Işıklar kapalı · Önizleme açık';
  try {
    await pendingWrite;
    if (keyboard.device && keyboard.version) await keyboard.write(matrix.flat().map(() => [0, 0, 0]), matrix);
  } catch (error) { message = error.message; }
  finally { if (disconnect) { await keyboard.close().catch(() => {}); message += ' · Bağlantı kesildi'; } busy = false; state(message); }
}
$('stop').onclick = () => stop();
$('disconnect').onclick = () => stop(true);
navigator.hid?.addEventListener('disconnect', event => { if (event.device === keyboard.device) { running = false; keyboard.device = null; keyboard.version = null; state('Klavye çıkarıldı · Önizleme açık'); } });
function animate(now) {
  const frame = renderFrame(matrix, $('effect').value, now / 1000, $('brightness').value, $('color').value);
  frame.forEach((rgb, i) => { keys[i].style.backgroundColor = `rgb(${rgb.join(',')})`; });
  if (running && !busy && !writing && now - lastWrite >= 50) {
    writing = true; lastWrite = now;
    pendingWrite = keyboard.write(frame, matrix).catch(error => { running = false; state(`Gönderim durdu: ${error.message}`); }).finally(() => { writing = false; });
  }
  requestAnimationFrame(animate);
}
requestAnimationFrame(animate);
