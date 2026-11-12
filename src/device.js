import { FILTERS, matchesDevice, versionRequest, parseVersion, modePacket, buildPackets } from './protocol.js';
export class Keyboard {
  async connect() {
    if (!navigator.hid) throw new Error('Chrome veya Edge kullanın; WebHID desteği gerekli.');
    const [device] = await navigator.hid.requestDevice({ filters: FILTERS });
    if (!device) return false;
    if (!matchesDevice(device)) throw new Error('Desteklenen RS7 Air arabirimi seçilmedi.');
    this.device = device;
    try {
      await device.open();
      this.maxPayload = Math.max(0, ...device.collections.filter(c => c.usagePage === 0xff60 && c.usage === 0x61).flatMap(c => c.outputReports || []).filter(r => r.reportId === 0).map(r => r.items.reduce((sum, item) => sum + item.reportSize * item.reportCount, 0) / 8));
      const version = await new Promise((resolve, reject) => {
        const cleanup = () => { clearTimeout(timer); device.removeEventListener('inputreport', handler); };
        const handler = event => { if (event.reportId !== 0) return; const v = parseVersion(event.data); if (v) { cleanup(); resolve(v); } };
        const timer = setTimeout(() => { cleanup(); reject(new Error('Firmware sürümü okunamadı. ATK HUB kapalıyken tekrar deneyin.')); }, 2500);
        device.addEventListener('inputreport', handler);
        device.sendReport(0, versionRequest()).catch(error => { cleanup(); reject(error); });
      });
      this.version = version;
      return true;
    } catch (error) { await this.close(); throw error; }
  }
  async start(matrix) {
    buildPackets(matrix.flat().map(() => [0, 0, 0]), matrix, this.version.code, this.maxPayload);
    if (this.version.code < 200) {
      if (this.maxPayload < 63) throw new Error('Aydınlatma raporu desteklenmiyor.');
      await this.device.sendReport(0, modePacket());
    }
  }
  async write(frame, matrix) {
    for (const packet of buildPackets(frame, matrix, this.version.code, this.maxPayload)) await this.device.sendReport(0, packet);
  }
  async close() { if (this.device?.opened) await this.device.close(); this.device = null; this.version = null; }
}
