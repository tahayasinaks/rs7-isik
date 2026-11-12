# RS7 Işık

ATK RS7 Air için Türkçe aydınlatma uygulaması. İlk sürüm yerel olarak Chrome/Edge içinde çalışır; tasarım geçicidir. ATK ile bağlantılı resmî bir proje değildir.

**Durum:** Efekt motoru ve paket oluşturma test edildi. Fiziksel klavye, firmware yanıtı ve RGB aktarımı henüz doğrulanmadı. Çalışan donanım desteği veya tamamlanmış ürün olarak yayımlanmamalıdır.

## Çalıştırma

Node.js 22 veya üzeri ile bu klasörde `npm start` çalıştırın. Chrome veya Edge ile <http://127.0.0.1:4177> adresini açın. Bağımlılık kurulumu gerekmez.

1. Klavyeyi USB ile bağlayın, ATK HUB ve diğer RGB uygulamalarını kapatın.
2. **Klavyeyi bağla** ile tarayıcının cihaz seçicisinden RS7 Air’i seçin.
3. Firmware sürümü okunursa **Klavyede başlat** düğmesine basın.
4. Işık modu ve parlaklığı değiştirin.
5. **Durdur ve ışıkları kapat**, LED’lere siyah gönderir. **Bağlantıyı kes** de önce siyah gönderir, ardından HID bağlantısını kapatır.

Uygulama açıkken efektler bilgisayarda hesaplanır. Klavye belleğine yeni firmware/kalıcı efekt kurulmaz. Sekme kapanınca veya arka planda kalınca animasyon durabilir; son renk cihazda kalabilir. Önceki ATK aydınlatma ayarı otomatik geri yüklenmez. ATK HUB üzerinden yeniden seçilebilir.

## Efektler

Öncelikli Dalga, Çapraz dalga, Gökkuşağı, Aurora, Ateş, Yağmur, Yıldızlar, Nefes, Sabit renk ve Kapalı. Parlaklık %0–100 aralığında gönderilen RGB renklerine uygulanır. Renk seçimi Nefes/Sabit renk için geçerlidir. Tuşa tepki efektleri, doğrulanmış tuş olay desteği gelene kadar eklenmemiştir.

## Geliştirme

`npm test` efektler, paket bölme, matris sırası ve cihaz doğrulama kontrollerini çalıştırır. `.github/workflows/check.yml` aynı kontrolleri GitHub Actions üzerinde çalıştırmaya hazırdır.

- `src/effects.js`: donanımdan bağımsız efekt motoru
- `src/protocol.js`: RS7 Air filtreleri ve RGB paketleri
- `src/device.js`: WebHID bağlantısı ve sıralı gönderim
- `docs/ARASTIRMA.md`: kaynaklar, doğrulanan bulgular ve açık noktalar
- `docs/YOL-HARITASI.md`: geliştirme ve yayın işleri

ANSI model hedeflenir. ISO varyantı ayrı matris doğrulaması gerektirdiği için bağlanma listesine alınmadı. USB kimliği listede bulunmayan cihazlara komut gönderilmez.
