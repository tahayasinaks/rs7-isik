# RS7 Light — Kurulum ve güncelleme

RS7-Light-v1.2.0-preview.8-Setup-x64.exe bir kurulum önizlemesidir; eski RS7'yi sistem tepsisi menüsünden Tamamen çık ile kapattıktan sonra açın. Kurulum kullanıcı hesabınıza yapılır; yönetici yetkisi gerektirmez. Başlat menüsü, masaüstü kısayolu ve kaldırıcı eklenir. Eski ışık/pencere ayarları korunur.

## Otomatik başlatma

Windows ile başlat açıldığında mevcut ayarlar kaydedilir, USB bağlantısı ve efekt otomatik başlatılır. Sonraki uygulama açılışlarında da bu tercih açıksa aynı işlem yapılır. USB aygıtı henüz hazır değilse iki dakika boyunca iki saniye aralıkla yeniden denenir. Durdur/Bağlantıyı kes gibi elle verilen komutlar denemeyi iptal eder.

Ayar değişiklikleri 750 ms durulduktan sonra kaydedilir; tam çıkışta da kaydedilir. Önizleme EXE'si (Onizleme adlı dosya veya --design-preview) gerçek USB/başlangıç ayarını değiştirmez.

## Güncelleme akışı

Uygulama açılırken arka planda yalnızca projenin GitHub Releases en son kararlı sürümü kontrol edilir. Sürüm numarası mevcut sürümden büyük olmalıdır; eski/farklı sürümler güncelleme sayılmaz. Taslak ve önizleme yayınları alınmaz. Aynı sürümdeki hotfix için yeni bir sürüm numarası yayımlanmalıdır.

Yeni sürüm bulunduğunda bildirim ve İndir ve kur seçeneği sunulur. İndirme onayınızdan önce başlamaz. İnternet kesilmesi uygulamayı veya ışık akışını durdurmaz; tekrar deneme düğmesi vardır.

Kurulum dosyası adı RS7-Light-vX.Y.Z-Setup-x64.exe olmalıdır. Beklenen GitHub deposu/adresi, dosya boyutu, API SHA-256 özeti ve SHA256SUMS.txt eşleştirilir. Dosya tamamen indirilip aynı özetle doğrulanmadan çalıştırılmaz. Doğrulamadan sonra ayarlar kaydedilir, setup açılır; setup uygulama sürecinin kapanmasını bekler, aynı kurulum klasöründe dosyayı yeniler. Başlangıç tercihi korunur. Kurulum sonunda uygulamayı açma seçeneği vardır.

Özet doğrulaması yayın kaynağının bütünlüğünü kontrol eder; dijital kod imzası değildir. EXE ve setup henüz imzasızdır. Windows Akıllı Uygulama Denetimi engeli devam edebilir. GitHub hesabı/yayın süreci güvenliği de gereklidir.

## Test durumu

Otomatik testler sürüm sıralamasını, indirme adresi ve SHA alanlarının reddedilmesini, dosya adı/özet eşleşmesini kapsar. Temiz Windows CI ortamında kurulum, tekrar kurulum ve kaldırma test edilir; /TESTMODE ile kullanıcı başlangıç/kısayol/kayıtları değiştirilmez. Gerçek yeniden başlatma, klavyenin açılışta bağlantısı ve ilerideki bir kararlı sürüme canlı yükseltme ayrıca kullanıcı cihazında doğrulanmalıdır.

Kaldırma uygulama dosyalarını ve kısayollarını kaldırır; kullanıcı ışık ayarlarını silmez. İndirme önbelleği %LOCALAPPDATA%\RS7Light\Updates altındadır. Işık ayarları uyumluluk için %APPDATA%\RS7Isik altında tutulur.
