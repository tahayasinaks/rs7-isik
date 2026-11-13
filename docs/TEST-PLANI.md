# RS7 v1.1.3 — kullanıcı test planı

İndirilecek dosya: RS7-v1.1.3-Windows-x64.exe. Eski RS7 sürümünden tepsi menüsü → Tamamen çık ile çıkın; ATK HUB kapalı, klavye USB ile bağlı olsun.

| Kontrol | Beklenen sonuç |
|---|---|
| Pencere kenarını sürükle / Ekranı Kapla / Geri yükle | Klavye oranı ve tıklama eşlemesi korunur; kartlar ve ayarlar uygun boyuta geçer. |
| Dar pencere / kaydırma | Paneller alt alta, kartlar iki sütuna geçer; bütün kontroller kaydırmayla ulaşılabilir olur. |
| Renk kutusu → ton şeridi / HEX / RGB → Uygula | Yeni renk kutuya uygulanır. Vazgeç veya X önceki rengi korur; geçersiz giriş Uygula'yı kapatır. |
| Gökkuşağı / Tek renk / İki renk | Sırasıyla otomatik renk açıklaması / bir renk kutusu / iki renk kutusu görünür. |
| Windows ile başlat → Açık → oturumu kapatıp aç | RS7 penceresi açılır. Klavyeye bağlanma/başlatma elle yapılır. |
| Windows ile başlat → Kapalı → oturumu kapatıp aç | RS7 otomatik açılmaz. Sürüm dosyası taşındığında yeni konumda tercihi kapatıp açın. |
| Tamamen çık → yeniden aç: pencere | Boyut/konum geri gelir; kaybolan monitör varsa görünür alana uyarlanır. |
| Klavyeyi bağla → Başlat | Üstte USB/çalışma durumu görünür; klavye ışıkları seçilen efekte geçer. |
| Sabit renk → ilk renk kırmızı / yeşil / mavi | Klavye sırasıyla seçilen renkte yanar. Bu modda ikinci renk ve hız pasiftir. |
| Parlaklık %0 → %30 → %100 | Işıklar söner; sonra kısık ve tam parlaklıkta yanar. |
| Sürekli dalga → Tek renk → ilk renk seç | Tek renkte de hareketli parlaklık dalgası görülür; düz renge dönüşmez. |
| Deniz dalgası / Spiral → hız 0.20× / 3.00× | Hareket hızı belirgin şekilde değişir. |
| İki renk → ilk ve ikinci renkleri farklı seç | Desen iki renk arasında geçiş yapar; ayarlar çalışırken uygulanır. |
| Arama: kalp → Kalp şekli | Tuşlar kalp silueti oluşturur; çevre karanlıktır. Kalp atışı ayrı olarak bütün klavye parlaklığını değiştirir. |
| Tuşa basınca dalga / Tek tuş sönümü / Tuş alevi | Tuşa basınca ilgili tepki görülür; basmadan sürekli yeni tepki oluşmaz. Fn algılanmaz. |
| Tuşla tsunami → A / sağ taraftaki bir tuş | Klavye boyunca dik dalga sırasıyla sağa / sola ilerler; renk, hız ve parlaklık uygulanır. |
| Görsel kartların üzerine gel → başka karta geç | Önizleme hareket eder; tıklayınca mod ve açıklama güncellenir. Büyük klavye ve sağ ayarlar yerinde kalır. |
| Kategori filtresi / arama / sayfalar | Liste uygun sonuçlara iner; boş aramada açıklama görünür; önceki/sonraki sayfalar doğru değişir. |
| Ses modları → Windows varsayılan çıkışında müzik çal | Ses seviyesiyle ışıklar değişir. Mikrofon açılmaz. Özel ses kipleri ölçülemeyebilir. |
| Ekran modları → farklı renkli pencere göster | Birincil ekran renkleri yansır; palet yerine ekran renkleri kullanılır. |
| İşlemci yükü / Bellek doluluğu / Saat | Windows kullanım oranları / yerel saat görünür; sıcaklık ölçümü değildir. |
| X ile kapat → tepsi simgesinden Aç | Işık devam eder ve pencere tekrar açılır. |
| Durdur → Başlat | Durdurunca ışık akışı durur; yeniden başlatınca seçili efekt döner. |
| Tamamen çık → yeniden aç | Eski mod, renk, hız ve parlaklık hatırlanır; USB'ye otomatik bağlanmaz. |

USB kablosunu çıkarıp tekrar takarsanız bağlantı hatası görünmelidir; yeniden Klavyeyi bağla seçin.

Sorun bildirirken: sürüm, seçilen efekt/palet, hız/parlaklık, beklenen ve görülen davranış, Windows sürümü ve uygulamadaki hata mesajını yazın. Modlar arasında fark olup olmadığını belirtin. Tuş/ses/ekran kaydı paylaşmanız gerekmez.

Bu sürümde 28 otomatik test geçti; 1 canlı kaynak testi varsayılan olarak atlanır. Oturum açılışı, tüm Windows ölçekleme oranları ve bütün modların fiziksel cihaz testi henüz tamamlanmadı. EXE imzasızdır; Akıllı Uygulama Denetimi engeli bu sürümde devam edebilir.
