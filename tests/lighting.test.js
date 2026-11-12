import test from 'node:test';
import assert from 'node:assert/strict';
import { readFile } from 'node:fs/promises';
import { EFFECTS, renderFrame } from '../src/effects.js';
import { buildPackets, parseVersion, matchesDevice } from '../src/protocol.js';
const matrix = JSON.parse(await readFile(new URL('../src/layout.json', import.meta.url)));
test('bütün efektler geçerli 90 LED rengi üretir; sıfır parlaklık karartır', () => {
  for (const effect of Object.keys(EFFECTS)) {
    for (const time of [0, .5, 2, 20]) {
      const frame = renderFrame(matrix, effect, time, 100);
      assert.equal(frame.length, 90);
      assert.ok(frame.flat().every(v => Number.isInteger(v) && v >= 0 && v <= 255));
      assert.ok(renderFrame(matrix, effect, time, 0).flat().every(v => v === 0));
    }
  }
});
test('dalga zaman ve yatay konumla değişir', () => {
  const first = renderFrame(matrix, 'wave', 0, 100);
  assert.notDeepEqual(first[0], first[1]);
  assert.notDeepEqual(first, renderFrame(matrix, 'wave', 1, 100));
});
test('eski firmware üç paket, yeni firmware bir paket kullanır; renk sırası korunur', () => {
  const frame = Array.from({ length: 90 }, (_, i) => [i, 255 - i, 12]);
  const packets = buildPackets(frame, matrix, 110, 127);
  assert.deepEqual(packets.map(p => p[3]), [41, 41, 8]);
  assert.deepEqual([...packets[1].slice(0, 7)], [29, 2, 11, 41, 41, 214, 12]);
  assert.deepEqual(packets.flatMap(p => [...p.slice(4)]), frame.flat());
  assert.equal(buildPackets(frame, matrix, 200, 339).length, 1);
  assert.throws(() => buildPackets(frame, matrix, 200, 63));
  assert.throws(() => buildPackets(frame.slice(1), matrix, 110, 127));
});
test('firmware yanıtı ve cihaz arabirimi doğrulanır', () => {
  const version = Uint8Array.from([2, 150, 0, 0, 0, 1, 0x10, 0]);
  assert.deepEqual(parseVersion(new DataView(version.buffer)), { code: 110, label: '1.10' });
  assert.equal(parseVersion(new DataView(new ArrayBuffer(2))), null);
  assert.equal(parseVersion(new DataView(new Uint8Array([7, 0, 0, 0, 0, 1, 0, 0]).buffer)), null);
  const device = { vendorId: 0x373b, productId: 4640, collections: [{ usagePage: 0xff60, usage: 0x61 }] };
  assert.equal(matchesDevice(device), true);
  assert.equal(matchesDevice({ ...device, productId: 4801 }), false);
  assert.equal(matchesDevice({ ...device, collections: [{ usagePage: 1, usage: 6 }] }), false);
});
