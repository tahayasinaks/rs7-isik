import test from 'node:test';
import assert from 'node:assert/strict';
import { renderFrame } from '../src/effects.js';
import { positionForCode, positionForScanCode, Ripples } from '../src/reactive.js';
const matrix = Array.from({length:6}, (_, y) => Array.from({length:15}, (_, x) => ({x,y})));
test('tuş olmadan karanlık; basılan konumdan halka yayılır ve söner', () => {
  const ripple = [{x:5,y:2,time:10}];
  assert.ok(renderFrame(matrix,'reactive',10,100).flat().every(v => v === 0));
  const initial = renderFrame(matrix,'reactive',10,100,'#ffffff',ripple);
  assert.ok(initial[35].some(v => v > 0));
  assert.deepEqual(initial[41], [0,0,0]);
  const later = renderFrame(matrix,'reactive',10.5,100,'#ffffff',ripple);
  assert.deepEqual(later[35], [0,0,0]);
  assert.ok(later[39].some(v => v > 0));
  assert.ok(renderFrame(matrix,'reactive',13,100,'#ffffff',ripple).flat().every(v => v === 0));
  assert.ok(renderFrame(matrix,'reactive',10.5,0,'#ffffff',ripple).flat().every(v => v === 0));
});
test('fiziksel ve tarayıcı tuşları aynı LED konumuna eşleşir', () => {
  for (const [scan, code] of [[30,'KeyA'],[57,'Space'],[328,'ArrowUp'],[54,'ShiftRight'],[339,'Delete'],[43,'Backslash']]) assert.deepEqual(positionForScanCode(scan),positionForCode(code));
  assert.equal(positionForScanCode(999),null);
  assert.equal(positionForCode('Unidentified'),null);
});
test('eşzamanlı halkalar birleşir; olay kuyruğu sınırlıdır ve temizlenir', () => {
  const queue = new Ripples();
  for (let i=0;i<100;i++) queue.add({x:i%15,y:0},10);
  assert.equal(queue.events.length,32);
  assert.equal(queue.active(13).length,0);
  queue.clear(); assert.equal(queue.events.length,0);
  const frame = renderFrame(matrix,'reactive',10,100,'#fff',[{x:1,y:1,time:10},{x:12,y:4,time:10}]);
  assert.ok(frame[16].some(v=>v>0)); assert.ok(frame[72].some(v=>v>0));
});
