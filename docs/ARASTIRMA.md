# Araştırma — 12 Kasım 2026

## Birincil kaynaklar

- Resmî sürücü merkezi: https://desktop.atkgear.com/
- Resmî web sürücüsü: https://v3-hub.atkgear.com/
- İncelenen model ve RGB akış tanımları: https://bpcdn.atkgear.com/hub-v3/production/3.2.32/static/demoMode.store-afQcByDx.js
- Diğer klavye protokolleri: https://bpcdn.atkgear.com/hub-v3/production/3.2.32/static/keyboard-info.util-DhDItlI-.js

Model bilgileri ve paket biçimleri yayımlanmış istemci dosyalarından incelendi. Üreticinin uygulama kodu projeye paketlenmedi; taşıma ve efekt motoru ayrı yazıldı. Bunlar belgelenmiş ve üretici tarafından destek taahhüdü verilmiş bir SDK değildir. İstemci güncellenince URL’ler değişebilir.

## RS7 Air için gözlemler

- Vendor ID: `0x373b`; aydınlatma arabirimi usage page `0xff60`, usage `0x61`.
- RGB model filtresinde ürün kimlikleri: 4640, 4641, 4651, 4695, 4696, 4921, 4917, 4888. Bunlar sürücüdeki eşleşmelerdir; kullanıcının fiziksel cihaz kimliği henüz okunmadı.
- ANSI matris: 6 satır × 15 konum. Gizli konumlar ve boşluk tuşunun birden çok LED’i dahil toplam 90 renk aktarılır. Son satırdaki x=12 ve x=11 sıralaması kaynak matrise göre korunur.
- RGB888 akışı: report ID 0, payload `[0x1d, başlangıçSatırı, başlangıçSütunu, ledSayısı, R,G,B,...]`.
- Kaynak, firmware kodu 200 altındayken pakette 41 LED; 200 ve üzerinde 112 LED sınırı kullanıyor. Uygulama HID descriptor kapasitesini ayrıca kontrol eder.
- Sürüm isteği: report ID 0, 63 bayt payload, ilk baytlar `[2,150]`. Kaynaktaki channel-60 okuyucusu gelen komutun 2 olmasını kontrol edip yanıtın 5/6 konumlarından sürümü çıkarır. WebHID üzerinde aynı yanıt düzeni fiziksel test bekliyor.
- Firmware 200 öncesinde RGB moduna geçiş: report ID 0, 63 bayt payload; başlangıcı `[7,65,46,0,127,255,0,255,100]`. Yeni firmware için kaynak bu geçişi atlıyor.
- Parlaklık ilk sürümde RGB kanallarını ölçekleyerek uygulanır. Böylece aydınlatma dışındaki yapılandırma alanlarını değiştirmeye gerek kalmaz.

## Açık noktalar

1. Kullanıcının VID/PID, ANSI/ISO düzeni ve firmware sürümü.
2. Gerçek WebHID rapor kapasitesi ve sürüm yanıtı.
3. Dalga yönü, bütün LED’lerin eşleşmesi, parlaklık ve kapatma davranışının fiziksel doğrulaması.
4. Kalıcı efekt yükleme, önceki efekti geri yükleme ve uygulama kapalıyken çalışma bu sürümde yok.
5. Açık RS7 Air SDK’sı veya hazır OpenRGB desteği bu araştırmada doğrulanamadı. Arama sonucu bulunmaması bunların kesinlikle bulunmadığı anlamına gelmez.

Windows cihaz envanteri okuma girişimi, bu oturumda CIM erişimi olmadığı için sonuç vermedi. Klavyenin bağlı olup olmadığı bu nedenle doğrulanmadı.
