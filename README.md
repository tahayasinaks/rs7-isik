# RS7 Işık

ATK RS7 Air için Türkçe aydınlatma uygulaması. Windows EXE ve yerel Chrome/Edge sürümleri; tasarım geçicidir. ATK ile bağlantılı resmî bir proje değildir.

**Durum:** İlk web sürümünün klavyede çalıştığı kullanıcı tarafından doğrulandı. Windows EXE ve yeni tuşa tepki modu ayrıca fiziksel test gerektirir.

## Windows uygulaması

`RS7-Isik-0.2.0-Windows-x64.zip` dosyasını çıkarıp içindeki `RS7 Işık.exe` uygulamasını açın. EXE ile yanındaki kaynak ve DLL dosyaları aynı klasörde kalmalıdır; kurulum gerekmez. Node.js veya ayrı tarayıcı kurulumu gerektirmez. Dijital imzası yoktur. EXE açıldığında Klavyeyi bağla → Tuşa basınca dalga → Klavyede başlat sırasını izleyin.

Bu modda bastığınız tuştan renkli halka yayılır ve söner. Aynı anda birden çok halka desteklenir. Windows tuş dinleyicisi yalnızca tuşa tepki modu çalışırken açılır; mod değiştirince, durdurunca, USB çıkarılınca veya uygulama kapanınca kapanır. Tuşlar diske kaydedilmez ve ağ üzerinden gönderilmez. Fiziksel tarama kodu kullanılır; Windows yazı dilinden bağımsızdır. Diğer bağlı klavyeler de tetikleyebilir; Fn algılanmaz. Uygulama açıkken diğer pencerelerde de çalışır.

Kaynak koddan Windows üzerinde: `npm ci`, `npm run desktop`; EXE üretmek için `npm run dist:win`. Yardımcı program Windows .NET Framework C# derleyicisi ile oluşturulur. Windows yürütme ilkeleri değiştirilmez.

## Tarayıcıda çalıştırma

Node.js 22 veya üzeri ile bu klasörde `npm start` çalıştırın. Chrome veya Edge ile <http://127.0.0.1:4177> adresini açın. Bağımlılık kurulumu gerekmez.

1. Klavyeyi USB ile bağlayın, ATK HUB ve diğer RGB uygulamalarını kapatın.
2. **Klavyeyi bağla** ile tarayıcının cihaz seçicisinden RS7 Air’i seçin.
3. Firmware sürümü okunursa **Klavyede başlat** düğmesine basın.
4. Işık modu ve parlaklığı değiştirin.
5. **Durdur ve ışıkları kapat**, LED’lere siyah gönderir. **Bağlantıyı kes** de önce siyah gönderir, ardından HID bağlantısını kapatır.

Uygulama açıkken efektler bilgisayarda hesaplanır. Klavye belleğine yeni firmware/kalıcı efekt kurulmaz. Sekme kapanınca veya arka planda kalınca animasyon durabilir; son renk cihazda kalabilir. Önceki ATK aydınlatma ayarı otomatik geri yüklenmez. ATK HUB üzerinden yeniden seçilebilir.

## Efektler

Tuşa basınca dalga, Sürekli dalga, Çapraz dalga, Gökkuşağı, Aurora, Ateş, Yağmur, Yıldızlar, Nefes, Sabit renk ve Kapalı. Parlaklık %0–100 aralığında gönderilen RGB renklerine uygulanır. Renk seçimi Nefes/Sabit renk için geçerlidir. Tarayıcı sürümünde tuşa tepki yalnızca sayfa odaktayken çalışır.

## Geliştirme

`npm test` efektler, paket bölme, matris sırası ve cihaz doğrulama kontrollerini çalıştırır. `.github/workflows/check.yml` aynı kontrolleri GitHub Actions üzerinde çalıştırmaya hazırdır.

- `desktop/`: Electron kabuğu, dar kapsamlı IPC ve Windows tuş dinleyicisi
- `src/reactive.js`: fiziksel tuş/LED eşlemesi ve sınırlı halka kuyruğu
- `src/effects.js`: donanımdan bağımsız efekt motoru
- `src/protocol.js`: RS7 Air filtreleri ve RGB paketleri
- `src/device.js`: WebHID bağlantısı ve sıralı gönderim
- `docs/ARASTIRMA.md`: kaynaklar, doğrulanan bulgular ve açık noktalar
- `docs/YOL-HARITASI.md`: geliştirme ve yayın işleri

ANSI model hedeflenir. ISO varyantı ayrı matris doğrulaması gerektirdiği için bağlanma listesine alınmadı. USB kimliği listede bulunmayan cihazlara komut gönderilmez.
