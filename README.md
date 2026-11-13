# RS7 Light

ATK RS7 Air için Türkçe, yerel Windows RGB uygulaması. Rust ile yazılmıştır. Setup ve taşınabilir EXE seçenekleri vardır; Rust, .NET veya Electron kurulumu gerektirmez.

[**Son sürümü indir**](https://github.com/tahayasinaks/rs7-isik/releases/latest) · [66 efektin kataloğu](docs/EFEKT-KATALOGU.md) · [Test planı](docs/TEST-PLANI.md)

![RS7 arayüzü](docs/images/rs7.png)

## v1.2.0 hotfix

RS7 Light artık kullanıcı hesabına kurulan bir setup ile dağıtılır. Masaüstü/Başlat menüsü kısayolları ve kaldırıcı içerir; ayarlar korunur. [Kurulum ve güncelleme kılavuzu](docs/RS7-LIGHT-KURULUM.md).

**Windows ile başlat** açıkken son mod, hız, parlaklık ve renklerle klavyeye otomatik bağlanıp efekti başlatır. Aygıt henüz hazır değilse iki dakika yeniden dener; elle durdurma denemeyi iptal eder. Ayar değişiklikleri kısa bir beklemeden sonra kaydedilir.

Açılışta daha yeni kararlı GitHub sürümü kontrol edilir. Yeni sürümde **İndir ve kur** bildirimi sunulur; indirme yalnızca seçiminden sonra başlar. Boyut, SHA-256 ve yayın doğrulama dosyası kontrol edilir; doğrulanan setup uygulamanın kapanmasını bekleyip aynı kurulum klasörünü günceller.

**Kalp şekli** referanstaki 24 tuşu kırmızı yakar: F5/F6/F8/F9; 4/5/6/7/8/9/0/−; R/T/Y/U/I/O/P; G/H/J/K; N. F7 ve diğer tuşlar karanlıktır. Hız ve parlaklık ayarlanır; renk bu modda kırmızıdır.

## Kullanım

1. Releases → **RS7-Light-v1.2.0-Setup-x64.exe** dosyasını indir. Eski RS7'den tepsi menüsü → **Tamamen çık** ile çık, ardından setup'ı aç.
2. Kurulumdan sonra **RS7 Light** kısayolunu aç; klavyeyi USB ile bağla ve ATK HUB'ı kapat.
3. Otomatik açılış istemiyorsan **Klavyeyi bağla → Başlat** seç. Otomatik açılış istiyorsan **Windows ile başlat** tercihini aç; o anda ve sonraki açılışlarda son ayarlarla otomatik bağlanıp başlar.
4. Efekt, renk, hız ve parlaklık çalışırken değiştirilebilir. **X / küçült** efekti tepside sürdürür; **Tamamen çık** USB'yi bırakır.
5. Kurulumsuz kullanım için **RS7-Light-v1.2.0-Windows-x64.exe** seçeneği de vardır.

## Özellikler

- 66 Türkçe seçenek; görsel kartlar, arama, yedi kategori ve altışarlı sayfalama.
- Dalga, deniz dalgası, kalp şekli, tuşa duyarlı desenler, doğal ve geometrik animasyonlar.
- Gökkuşağı, tek renk ve iki renk geçişi; %0–100 parlaklık ve 0.20×–3.00× hız. Tek renkli dalga ve geometrik desenler parlaklık geçişiyle hareketini korur.
- Windows ses çıkışına tepki (mikrofon açılmaz), birincil ekran renkleri, işlemci/bellek yükü ve saat.
- Üst barda duruma göre değişen bağlantı ve başlat/durdur düğmeleri.
- Şeffaf logo, 81 tuşlu canlı çizim; #0b0b0c / #ffffff / #c6f432 paleti.
- Tam çıkışta efekt/renk/hız/parlaklık kaydedilir; sonraki açılışta geri yüklenir. Otomatik başlangıç tercihi açıkken bağlantı ve efekt de otomatik başlar.
- Tek örnek, Explorer yeniden başlarsa tepsi simgesini geri ekleme; USB döngüsü arayüz çiziminden ayrıdır.

Bazı modlarda hız veya palet anlamlı değildir ve ilgili denetim pasiftir. Sabit renk birinci rengi kullanır. Ekran modları kendi ekran renklerini kullanır; parlaklık ayarlanabilir. Ses seviye çubukları kanal seviyesidir, frekans ekolayzırı değildir. CPU göstergesi yükü gösterir; sıcaklık ölçmez.

## Doğrulama ve sınırlar

31 otomatik Rust testi; efektlerin örnek çıktıları, parlaklık sınırı, tek renkte hareket, Türkçe arama, tuş/LED eşlemesi, USB paketleri, ayar kaydı ve gömülü logo yüklemesi için çalışır. Yerel Windows canlı kaynak entegrasyon testi ayrıca çalıştırılır; CI'da masaüstü/ses aygıtı gerektirdiği için varsayılan olarak atlanır.

Fiziksel test cihazı USB 373b:1220 / firmware 2.02 kullanan RS7 Air'dir. Önceki sürümlerde bağlantı, tam çıkış ve X ile tepside kesintisiz devam etme doğrulandı. Yeni 66 seçeneğin tümü gerçek cihazda tek tek doğrulanmış değildir; [test planıyla](docs/TEST-PLANI.md) kontrol edebilirsiniz. Diğer firmware varyantları henüz fiziksel test edilmedi.

Animasyonlar bilgisayarda hesaplanır; tamamen çıkıştan sonra klavye firmware'ine özel efekt yüklenmiş olmaz. Fn algılanmaz; Windows tuş olayları kullanıldığından başka klavyeler de tepkiyi tetikleyebilir. Tuşlar, ses ve ekran örnekleri dosyaya/ağa yazılmaz. HDR, korumalı video veya özel ses kiplerinde canlı kaynak kısıtları olabilir; ayrıntılar katalogdadır.

EXE dijital imzalı değildir; Windows Akıllı Uygulama Denetimi bazı bilgisayarlarda çalışmasını engelleyebilir. Bu hotfix imzalama sorununu çözmez. Açık kaynak lisansı kod imzasının yerini tutmaz. Proje [MIT lisanslıdır](LICENSE) ve ATK ile bağlantılı resmî bir ürün değildir.

## Derleme

Windows + Rust stable:

```text
cargo test --locked
cargo build --release --locked
```

MSVC: Windows SDK / Visual C++ build araçları. C/C++ çalışma zamanı statik bağlanır. GNU: uygun GNU bağlama araçları ve ikon kaynağı için windres gerekir. Çıktı: target/release/rs7-isik.exe. Kullanıcı bilgisayarında derleme araçları gerekmez.

EXE adında Onizleme bulunursa veya --design-preview argümanı verilirse USB'siz tasarım modu açılır; canlı kaynaklar örnektir, ayarlar yazılmaz. Releases dosyası gerçek klavyeyi yöneten sürümdür.

Kaynak: src/lighting.rs ve src/lighting/extended.rs efektler; src/hardware.rs HID; src/worker.rs gönderim; src/sources.rs canlı kaynaklar; src/win_ui.rs ve alt modüller arayüz. Eski Electron kaynakları tarihçe için korunur; Rust EXE bunları kullanmaz.

## Yayınlar

Sürüm sırası v1.0.0 → v1.0.1 → v1.1.0 → v1.1.1 → v1.1.2 → v1.1.3 → **v1.2.0**. Her yayın EXE, SHA-256, Türkçe notlar ve kaynak etiketi içerir. [Yayınlama kılavuzu](docs/YAYINLAMA.md).
