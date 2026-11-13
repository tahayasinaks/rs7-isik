# Türkçe efekt kataloğu — 66 seçenek

Araştırma tarihi: 13 Kasım 2026. Sürüm: 1.2.0.

Bu katalog, yaygın RGB yazılımlarının belgelenmiş davranışlarını karşılaştırır. Marka adları yerine davranışa göre Türkçe adlar kullanılır. Bütün animasyon hesaplamaları bu proje için Rust ile yazıldı; üretici yazılımı veya ücretli efekt dosyaları pakete alınmadı. Üreticinin birebir görsel çıktısını veya belirli bir firmware efektini kopyalama iddiası yoktur.

## Araştırma kaynakları

- [Razer Chroma Studio efekt açıklamaları](https://mysupport.razer.com/app/answers/detail/a_id/13713): temel, tuşa duyarlı, ateş, yıldız ve dönen çark aileleri.
- [CORSAIR iCUE aydınlatma kılavuzu](https://help.corsair.com/hc/en-us/articles/360045171512-iCUE-How-to-Set-up-device-lighting-in-CORSAIR-iCUE): dalga, spiral, tarayıcı ve tuş tepkisi aileleri.
- [OpenRGB Effects Plugin kataloğu](https://gitlab.com/OpenRGBDevelopers/OpenRGBEffectsPlugin/-/raw/master/README.md): geometrik desenler, akışlar ve canlı kaynak grupları.
- [OpenRGB özellik açıklamaları](https://openrgb.org/plugin_effects.html): ses seviyesi ve ekran ortam ışığı.
- [SignalRGB efekt kütüphanesi](https://signalrgb.com/rgb/): dalga, matris, plazma ve ortam örnekleri; sürekli büyüyen özel içerik.
- [MSI Mystic Light](https://www.msi.com/support/technical_details/VGA_MSI_Utility_MysticLight): sabit, nefes, flaş ve birikme davranışları.
- [SteelSeries Engine kılavuzu](https://downloads.steelseriescdn.com/drivers/userguides/SteelSeriesEngine_3_1_User_Guide.pdf): sabit, nefes ve renk döngüsü.
- [ASUS Aura kılavuzu](https://www.asus.com/campaign/aura/upload/download/20180223060220_m1.pdf): kuyruklu yıldız, yıldız, dalga ve ses aileleri.

## Aynı davranışı veren adların birleştirilmesi

| Kaynaklarda kullanılan adlar | Uygulamadaki karşılığı |
|---|---|
| Static / Steady / Solid / Fixed | Sabit renk |
| Breathing / Breathe / Color pulse | Nefes |
| Spectrum Cycling / Color Cycle / ColorShift | Renk döngüsü |
| Rainbow Wave / Color Wave / Gradient Wave | Sürekli dalga; Çapraz dalga mevcut yön varyantıdır |
| Reactive / Type lighting per key / Fade | Tek tuş sönümü |
| Ripple / Type lighting ripple | Tuşa basınca dalga |
| Starlight / Starry Night / Twilight | Yıldızlar |
| Visor / Scanner | Gidip gelen tarayıcı |
| Comet / Meteor | Kuyruklu yıldız |
| Marquee / Chase / Custom Marquee | Işık kovalamacası |
| Ambient / Ambilight / Screen Sampler | Ekran yansıtma / Ekran ortam rengi |

Yalnızca farklı ad taşıyan aynı temel hareket için ek bir mod oluşturulmadı. Aynı ailedeki farklı uzamsal davranışlar (örneğin ince tuş halkası ve dolu tuş patlaması) ayrı kaldı. İlk 12 kimlik korunarak eski kayıtlı ayarlar bozulmadı; görünen Gökkuşağı ve Aurora adları Renk döngüsü ve Kutup ışıkları olarak Türkçeleştirildi.

## Uygulanan seçenekler

| Kimlik | Ad | Kategori | Davranış |
|---|---|---|---|
| 0 | Tuşa basınca dalga | Tuşa duyarlı | Basılan tuştan dışarı doğru ince halkalar yayılır. |
| 1 | Sürekli dalga | Akış ve dalga | Renk şeridi klavyenin bir ucundan diğerine akar. |
| 2 | Çapraz dalga | Akış ve dalga | Dalga satır ve sütunlarda çapraz ilerler. |
| 3 | Renk döngüsü | Temel | Bütün tuşlar renk tayfında birlikte dolaşır. |
| 4 | Kutup ışıkları | Doğa | Yumuşak renk perdeleri üst üste akar. |
| 5 | Ateş | Doğa | Alevler alttan yükselir ve düzensiz titreşir. |
| 6 | Yağmur | Doğa | Bağımsız damlalar sütunlardan aşağı süzülür. |
| 7 | Yıldızlar | Doğa | Dağınık ışık noktaları yavaşça belirip kaybolur. |
| 8 | Nefes | Temel | Parlaklık yumuşakça artar ve azalır. |
| 9 | Sabit renk | Temel | Seçilen birinci renk bütün tuşlarda sabit kalır. |
| 10 | Kapalı | Temel | Uygulama bağlı kalır; bütün tuş ışıkları söner. |
| 11 | Deniz dalgası | Doğa | Geniş su dalgası ve ince köpük tepesi ilerler. |
| 12 | Tek tuş sönümü | Tuşa duyarlı | Yalnızca bastığınız tuş aydınlanır ve söner. |
| 13 | Tuş artısı | Tuşa duyarlı | Basılan tuşun satır ve sütununda ışık yayılır. |
| 14 | Tuş alevi | Tuşa duyarlı | Basılan tuştan yukarı doğru alev yükselir. |
| 15 | Tuş patlaması | Tuşa duyarlı | Basılan tuştan büyüyen dolu bir ışık alanı yayılır. |
| 16 | Klavye tepkisi | Tuşa duyarlı | Her tuş vuruşunda bütün klavye parlar ve söner. |
| 17 | Yazım ısı haritası | Tuşa duyarlı | Son vuruşlar birikir; sık basılan tuşlar daha parlak olur. |
| 18 | Rastgele renk | Temel | Bütün klavye düzenli aralıklarla yeni bir renge geçer. |
| 19 | Yanıp sönme | Temel | Bütün tuşlar aralıklı olarak açılıp kapanır. |
| 20 | Çift flaş | Temel | Her döngüde iki kısa ışık darbesi oluşur. |
| 21 | Kalp atışı | Temel | İki yumuşak darbeden sonra uzun bir bekleme gelir. |
| 22 | Tek yön tarama | Akış ve dalga | Dar bir ışık bandı bir yönde tarar; arkası söner. |
| 23 | Gidip gelen tarayıcı | Akış ve dalga | Işık bandı iki kenar arasında gidip gelir. |
| 24 | Kuyruklu yıldız | Akış ve dalga | Parlak bir baş ve sönümlenen kuyruk satırlarda ilerler. |
| 25 | Işık kovalamacası | Akış ve dalga | Aralıklı ışık grupları birbirini takip eder. |
| 26 | Dolup boşalma | Akış ve dalga | Tuş alanı adım adım dolar, sonra boşalır. |
| 27 | Biriken bloklar | Akış ve dalga | Hareketli bloklar kenarda üst üste birikir. |
| 28 | Renk takası | Geometri | İki dama grubu sırayla renklerini değiştirir. |
| 29 | Renk çarkı | Geometri | Renkler klavye merkezi çevresinde döner. |
| 30 | Spiral | Geometri | Dönerek dışarı açılan renk sarmalı oluşur. |
| 31 | Merkezden halkalar | Geometri | Merkezden dışarı doğru eş merkezli renk halkaları akar. |
| 32 | Radar | Geometri | Tek bir dar ışın merkez çevresinde tarar. |
| 33 | Çapraz ışınlar | Geometri | İki bağımsız ışık bandı birbirinin içinden geçer. |
| 34 | Çift renk çarkı | Geometri | İki merkezdeki renk çarkı ters yönlerde döner. |
| 35 | Sıçrayan top | Geometri | Tek bir ışık topu iki eksende sekerek hareket eder. |
| 36 | Hareketli noktalar | Geometri | Birden fazla ışık noktası bağımsız yörüngeler izler. |
| 37 | Kabarcıklar | Doğa | Yükselen boş ışık halkaları büyür ve kaybolur. |
| 38 | Havai fişek | Doğa | Farklı merkezlerden ışık parçacıkları patlar. |
| 39 | Şimşek | Doğa | Karanlık alan kısa ve düzensiz çatallanan flaşlarla parlar. |
| 40 | Gün doğumu | Doğa | Sıcak renkler aşağıdan yukarıya doğru yavaşça yükselir. |
| 41 | Mum ışığı | Doğa | Sıcak ve düşük parlaklıklı ışık yumuşakça titrer. |
| 42 | Lav | Doğa | Yavaş hareket eden parlak ve koyu sıcak lekeler oluşur. |
| 43 | Plazma | Geometri | Birbiriyle girişen sinüs alanları renkli desenler üretir. |
| 44 | Gürültü bulutu | Doğa | Yumuşak rastlantısal bulutlar hareket eder. |
| 45 | Mozaik | Geometri | Küçük renk blokları bağımsız aralıklarla yenilenir. |
| 46 | Hareketli paneller | Geometri | Geniş renk düzlemleri yatay ve dikey kayar. |
| 47 | Fraktal akış | Geometri | Çok ölçekli dalgalar kendini tekrar eden desenler üretir. |
| 48 | Dama | Geometri | Sabit ve dönüşümlü iki renkli kareler görünür. |
| 49 | Nefes alan halka | Geometri | Tek bir ışık halkasının çapı yumuşakça büyüyüp küçülür. |
| 50 | Çiçeklenme | Geometri | Merkezden açılan ışık yaprakları hareket eder. |
| 51 | Girdap halkaları | Geometri | İki merkez çevresinde bükülen halkalar döner. |
| 52 | Yılan | Akış ve dalga | Kuyruklu ışık izi tuşları satır satır dolaşır. |
| 53 | Sesle parlaklık | Ses ve ekran | Varsayılan ses çıkışının seviyesine göre bütün klavye parlar. |
| 54 | Ses seviye çubukları | Ses ve ekran | Sol ve sağ ses seviyeleri sütunlar halinde yükselir; frekans analizi değildir. |
| 55 | Sesle dalga | Ses ve ekran | Dalganın parlaklığı çalınan sesin seviyesine tepki verir. |
| 56 | Sesle yıldızlar | Ses ve ekran | Ses yükseldikçe daha fazla yıldız parlar. |
| 57 | Ekran yansıtma | Ses ve ekran | Birincil ekranın 15 × 6 renk haritası tuşlara yansır. |
| 58 | Ekran ortam rengi | Ses ve ekran | Birincil ekranın ortalama rengi bütün tuşlara uygulanır. |
| 59 | İşlemci yükü | Sistem | İşlemci kullanım oranı klavyede bir doluluk çubuğu oluşturur. |
| 60 | Bellek doluluğu | Sistem | Bellek kullanım oranı klavyede bir doluluk çubuğu oluşturur. |
| 61 | Saat | Sistem | Yerel saat ve dakika ışık noktalarıyla gösterilir. |
| 62 | Sesle parti | Ses ve ekran | Ses seviyesi renkli dama alanlarını aydınlatır. |
| 63 | Sesle halkalar | Ses ve ekran | Ses seviyesi merkezdeki halkanın boyutunu ve parlaklığını değiştirir. |

| 64 | Kalp şekli | Geometri | Referanstaki 24 tuş kırmızı kalp oluşturur; F7 karanlık, uç N tuşudur. Hız ve parlaklık ayarlanabilir. |

## Canlı kaynaklar ve kontroller

Ses modları Windows varsayılan ses çıkışının anlık tepe seviyelerini okur. Mikrofon açılmaz; PCM ses kaydı alınmaz. Ses seviye çubukları sol/sağ seviyedir, frekans ekolayzırı değildir. Windows paylaşımlı ses ölçeri kullanılır; bazı özel kip ses uygulamalarında sıfır seviye görülebilir. Varsayılan çıkış yaklaşık üç saniyede yenilenir.

Ekran modları birincil ekranı 15 × 6 renk alanına küçültür. Yaklaşık 8 örnek/sn; görüntü dosyaya veya ağa yazılmaz. HDR renkleri, korumalı video ve özel tam ekran durumları bu yöntemle doğru örneklenmeyebilir.

İşlemci yükü kullanım oranıdır, sıcaklık değildir. Bellek doluluğu Windows bellek kullanım yüzdesidir. Saat Windows yerel saatini gösterir.

Canlı kaynak yalnızca ilgili efekt başlatılınca çalışır. Örnekleme ayrı iş parçacığındadır; USB gönderiminin zamanlamasını bekletmez. Durdurma veya başka moda geçme, kullanılmayan kaynağı kapatır. Aynı anda tek seçili efekt ve tek USB gönderim sahibi vardır.

Parlaklık bütün modlarda uygulanır. Sabit/veriye bağlı desenlerde anlamlı olmayan hız denetimi pasiftir. Sabit renk birinci rengi kullanır. Ekran modlarında renk kaynağı ekran olduğundan renk/palet denetimleri pasiftir. Diğer desenlerde çok/tek/iki renk seçenekleri kullanılır.

Önizleme EXE'si ses, ekran ve sistem için örnek veri kullanır ve USB'ye bağlanmaz. Tuşa duyarlı önizlemede klavye çizimine tıklanabilir. Asıl EXE gerçek kaynakları kullanır; USB bağlantısı ve efekt başlatma kullanıcı eylemiyle yapılır.

## Kapsamın sınırı

İnternetteki bütün efektlerin sonlu ve tamamlanmış bir listesi yoktur. Özel betikler ve yeni topluluk efektleri sürekli eklenir. Bu 66 seçenek geniş bir başlangıç kütüphanesidir; bütün RGB uygulamalarındaki her özel içeriğin tamamlandığı anlamına gelmez. GIF/video oynatıcı, genel shader editörü, katman/mask düzenleyicisi ve oyunların canlı olay entegrasyonları ayrı ürün özellikleridir; bu listede varmış gibi gösterilmez. CPU/GPU sıcaklığı için sürücü veya harici sensör kaynağı eklenmedi.

## Doğrulama

20 otomatik test geçti. Bunlar 64 modun örneklenmiş çıktılarını karşılaştırır; ad/metadata tutarlılığı, üç palet ve üç hızda sıfır parlaklık, tuş tepkilerinin girdi olmadan karanlık kalması ve zamanla sönmesi, Türkçe İ/ı araması ve kategori/sayfa sınırları kontrol edilir. Farklı örnek çıktılar, bütün olası ayarlarda modların benzersizliğinin matematiksel kanıtı değildir.

Ayrıca yerel Windows ses ölçeri, ekran örnekleyicisi, işlemci/bellek ve saat kaynaklarını açıp okuyan bir entegrasyon testi geçti. CI ortamında masaüstü gerektirdiği için bu test varsayılan olarak atlanır. Arayüzde arama, seçim, sayfalama ve kategori filtresi incelendi. 64 modun tamamı gerçek RS7 üzerinde henüz tek tek denenmedi; bu yüzden sürüm önizlemedir.

## Tuşla tsunami

66. seçenek (ID 65), basılan tuşun fiziksel yatay konumundan başlayan dik dalgadır. Sol yarıdaki tuşlar sağa, sağ yarıdakiler sola dalga gönderir. Parlak tepenin arkasında sönümlenen geniş bir iz kalır. Her vuruş bağımsızdır; hız, parlaklık ve renk seçenekleri uygulanır.
