// PC scan codes identify physical positions independently of the Windows typing language.
const rows = [
  [1,59,60,61,62,63,64,65,66,67,68,87,88,338,339],
  [41,2,3,4,5,6,7,8,9,10,11,12,13,14,335],
  [15,16,17,18,19,20,21,22,23,24,25,26,27,43,329],
  [58,30,31,32,33,34,35,36,37,38,39,40,null,28,337],
  [42,44,45,46,47,48,49,50,51,52,53,54,null,328,null],
  [29,347,56,57,57,57,57,57,null,312,null,331,null,336,333]
];
export function positionForScanCode(scan) {
  if (scan === 57) return { x: 5, y: 5 };
  for (let y = 0; y < rows.length; y++) { const x = rows[y].indexOf(scan); if (x >= 0) return { x, y }; }
  return null;
}
const codes = [
  ['Escape','F1','F2','F3','F4','F5','F6','F7','F8','F9','F10','F11','F12','Insert','Delete'],
  ['Backquote','Digit1','Digit2','Digit3','Digit4','Digit5','Digit6','Digit7','Digit8','Digit9','Digit0','Minus','Equal','Backspace','End'],
  ['Tab','KeyQ','KeyW','KeyE','KeyR','KeyT','KeyY','KeyU','KeyI','KeyO','KeyP','BracketLeft','BracketRight','Backslash','PageUp'],
  ['CapsLock','KeyA','KeyS','KeyD','KeyF','KeyG','KeyH','KeyJ','KeyK','KeyL','Semicolon','Quote',null,'Enter','PageDown'],
  ['ShiftLeft','KeyZ','KeyX','KeyC','KeyV','KeyB','KeyN','KeyM','Comma','Period','Slash','ShiftRight',null,'ArrowUp',null],
  ['ControlLeft','MetaLeft','AltLeft',null,null,'Space',null,null,null,'AltRight',null,'ArrowLeft',null,'ArrowDown','ArrowRight']
];
export function positionForCode(code) {
  for (let y = 0; y < codes.length; y++) { const x = codes[y].indexOf(code); if (x >= 0) return { x, y }; }
  return null;
}
export class Ripples {
  events = [];
  add(position, time) {
    if (!position) return;
    this.events = this.active(time);
    this.events.push({ ...position, time });
    this.events = this.events.slice(-32);
  }
  active(time) { return this.events.filter(e => time >= e.time && time - e.time < 2.2); }
  clear() { this.events = []; }
}
