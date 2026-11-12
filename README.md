# RS7 Işık — v1.0.0

ATK RS7 Air için Türkçe, yerel Windows aydınlatma uygulaması. Arayüz, USB/HID gönderimi, efekt motoru ve tuş dinleme kodu Rust ile yazılmıştır. Electron, JavaScript, .NET veya ayrı yardımcı uygulama gerektirmez. Windows sistem DLL’lerini kullanır.

## Kullanım

`RS7-Isik-v1.0.0-Windows-x64.exe` dosyasını açın. Önce eski RS7 Işık uygulamasını ve ATK HUB’ı tamamen kapatın. **Klavyeyi bağla → mod seç → Başlat** sırasını izleyin.

- **Hız:** 0.20×–3.00×; çalışan efekte anında uygulanır.
- **Parlaklık:** %0–100.
- **Renk düzeni:** Gökkuşağı, Tek renk veya İki renk geçişi.
- **1. renk / 2. renk:** Windows renk seçicisini açar. Renk seçimi özel renk düzenini otomatik etkinleştirir. Sabit renk modu birinci rengi kullanır.
- **Tuşa basınca dalga:** basılan tuştan yayılıp sönen, üst üste binebilen renkli halkalar.
- **Deniz dalgası:** mavi/turkuaz deniz yüzeyi ve köpük gibi parlak dalga tepeleri. Hız ve özel renklerle ayarlanabilir.
- **Ayarları hatırlama:** tam çıkışta mod, renk, hız ve parlaklık kaydedilir; açılışta geri yüklenir. USB bağlantısı ve başlatma kullanıcı kontrolünde kalır.
- **Tek örnek:** ikinci açılış, mevcut pencereyi öne getirir.
- **Küçültme:** USB iş parçacığı çalışmaya devam eder.
- **X / küçültme:** sistem tepsisine gizler; efekt ve tuş dinleme devam eder. Simgeye tıklayarak geri açın. Sağ tık → Tamamen çık gerçek çıkıştır; akışı sonlandırır ve USB bağlantısını bırakır. Tepsi simgesi eklenemezse pencere gizlenmek yerine görev çubuğuna küçülür. Explorer yeniden başladığında simge geri eklenir.

Özel efektler bilgisayarda hesaplanır. Uygulama tamamen kapalıyken özel animasyon devam etmez; klavye firmware’ine efekt yüklenmez. Fn algılanmaz. Windows fiziksel tuş olayları kullanıldığı için başka klavyeler de tetikleyebilir. Tuşlar diske yazılmaz veya internete gönderilmez. Yönetici olarak çalışan bazı uygulamalarda Windows erişim sınırları nedeniyle tuş algılama çalışmayabilir.

## Mimari

`src/lighting.rs`: efekt, hız, palet ve parlaklık hesaplamaları; fiziksel tuş eşlemesi.

`src/hardware.rs`: yalnızca RS7 Air’in doğrulanan USB kimlikleri ve `ff60/61` arabirimi; sürüm okuma ve RGB paketleri.

`src/worker.rs`: 60 kare/sn hedefleyen, mutlak zamanla çalışan tek USB sahibi. Gecikmiş kareler biriktirilmez. Ayarlar en son değer olarak okunur; tuş kuyruğu sınırlıdır. Arayüz çizimi USB döngüsünü sürmez.

`src/win_ui.rs`: doğrudan Windows kontrolleri, renk seçicisi, önizleme ve Rust içindeki düşük seviyeli tuş kancası.

## Derleme

Windows ve Rust stable ile:

```text
cargo test --locked
cargo build --release --locked
```

MSVC hedefi Windows SDK/Visual C++ build araçlarını; GNU hedefi uygun GNU bağlama araçlarını gerektirir. EXE: `target/release/rs7-isik.exe`. Çalıştırmak için Rust kurulması gerekmez.

## Doğrulama

Hızın halka yarıçapına etkisi, özel renk ve parlaklık, halka yayılımı/sönmesi, tuş koordinatları, firmware yanıtı ve paket bölme ve ayar kaydı için 9 Rust testi vardır. Firmware 2.02 ve USB `373b:1220` kullanan RS7 Air’e yerel uygulamadan bağlantı kuruldu ve gönderim döngüsü çalıştırıldı. 0.3.0 testinde gerçek çıkıştan sonra ATK aydınlatmasının geri geldiği doğrulandı. v1.0.0 testinde kullanıcı, X ile tepsiye gizlendikten sonra Deniz dalgasının kesintisiz devam ettiğini doğruladı. Diğer firmware sürümleri henüz fiziksel test edilmedi.

Eski Electron sürümü kaynakları geçmişe uyumluluk için depoda durur; Rust EXE bunları kullanmaz. Önceki geliştirme notları: `docs/LEGACY-ELECTRON.md`. Proje ATK ile bağlantılı resmî bir ürün değildir. EXE dijital imzalı değildir.

## Yayın düzeni

Sürümler v1.0.0, v1.0.1, v1.1.0, v2.0.0 biçiminde artar. [Yayınlama kılavuzu](docs/YAYINLAMA.md) ve [v1.0.0 notları](releases/v1.0.0.md). GitHub Actions sürümü doğrulayıp testleri çalıştırır, EXE/SHA-256 dosyalarını Releases bölümünde yayımlar. Önceki 0.x sürümleri geliştirme geçmişidir.
