# Sürüm ve GitHub Releases düzeni

Tek sürüm kaynağı `Cargo.toml` paket sürümüdür. Pencere başlığı bu değerden üretilir. İlk ana yayın **v1.0.0**; **0.1.0, 0.2.0, 0.3.0** geliştirme geçmişi olarak korunur.

- Hata düzeltmesi: `1.0.0 → 1.0.1`.
- Yeni özellik: `1.0.1 → 1.1.0`.
- Büyük değişiklik: `1.x.x → 2.0.0 → 3.0.0`.

## Yeni yayın

1. `Cargo.toml` sürümünü artırın; `cargo check` ile `Cargo.lock` dosyasını güncelleyin.
2. `releases/vX.Y.Z.md` içine Türkçe notları yazın. Doğrulanmamış davranışları belirtin.
3. `cargo test --locked` ve `cargo build --release --locked` çalıştırın; USB, mod/hız/renk, X ile gizleme, geri açma ve Tamamen çık akışlarını fiziksel klavyede kontrol edin.
4. Değişiklikleri `main` dalına yükleyin.
5. GitHub → **Actions → Rust Windows Release → Run workflow**, sürüm alanı `X.Y.Z`. Alternatif: aynı commit üzerinde `vX.Y.Z` etiketi oluşturup gönderin.

İş akışı Cargo sürümünün etiketle eşleşmesini doğrular; test ve derlemeden sonra EXE ve SHA-256 dosyasını Releases bölümüne ekler. Var olan sürümü değiştirmez; düzeltme için yeni sürüm çıkarın. Yazma yetkisi yalnızca yayınlama işine verilir.

Her yayın: `RS7-Isik-vX.Y.Z-Windows-x64.exe`, `SHA256SUMS.txt`, Türkçe notlar ve kaynak etiketi. Kaynak ZIP/TAR dosyalarını GitHub otomatik sunar.
