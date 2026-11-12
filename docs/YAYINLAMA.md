# Sürüm ve GitHub Releases düzeni

Tek sürüm kaynağı Cargo.toml paket sürümüdür; Cargo.lock aynı sürümü içermelidir. Uygulamanın görünen adı RS7'dir.

- Düzeltme: 1.1.0 → 1.1.1.
- Yeni özellik: 1.1.0 → 1.2.0.
- Büyük değişiklik: 1.x.x → 2.0.0.

## Yayın hazırlama

1. Cargo.toml / Cargo.lock sürümünü artırın, releases/vX.Y.Z.md içine Türkçe notları yazın.
2. cargo test --locked ve cargo build --release --locked çalıştırın. Uygun fiziksel kontrolleri docs/TEST-PLANI.md ile yapın; yapılmayan kontrolleri notlarda belirtin.
3. Dosyaları main'e **release: RS7 vX.Y.Z** ile başlayan commit mesajıyla gönderin. Bu açık yayın işareti Windows yayın işini tetikler. Sıradan main commitleri Release oluşturmaz.
4. Alternatif: Actions → Rust Windows Release → Run workflow içinde X.Y.Z girin veya vX.Y.Z etiketi gönderin.

İş akışı talep/Cargo sürüm eşleşmesini ve Türkçe notları doğrular, test eder, MSVC ile statik çalışma zamanı kullanarak derler. RS7-vX.Y.Z-Windows-x64.exe, TEST-PLANI.md ve SHA256SUMS.txt dosyalarını yayımlar. Eski yayınların dosyalarını değiştirmez; düzeltme için yeni sürüm gerekir. GitHub kaynak ZIP/TAR dosyalarını otomatik sağlar.

Yayın işi yalnızca kendi görevinde contents:write yetkisi alır. Aynı anda tek yayın işi çalışır.
