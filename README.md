# PyCAN-Diag (Rust Core)

Automotive ECU network simulation and UDS (Unified Diagnostic Services) diagnostic interface core, implemented in **Rust** for memory safety and high performance.

## 🚀 Current Features
* **UDS Request Builder:** Object-oriented frame construction module supporting core UDS service IDs:
  * `0x10`: Diagnostic Session Control
  * `0x22`: Read Data By Identifier (DID)
  * `0x19`: Read DTC Information
  * `0x14`: Clear Diagnostic Information
* **Unit Testing:** Integrated test suite ensuring reliable frame serialization.

## 🛠️ Getting Started

To run the test suite and verify the core module:

```bash
cargo test


🗺️ My Roadmap
[x] Core UDS Request Builder & Unit Tests

[ ] ISO-TP Segmentation Layer (iso-dep)

[ ] Linux socketcan / Virtual CAN (vcan0) integration

[ ] ECU Simulation (Engine & Body ECUs)


----------------------------------------------------------------------------
🇹🇷 Türkçe
Bellek güvenliği ve yüksek performans için Rust dilinde geliştirilmiş otomotiv ECU ağ simülasyonu ve UDS (Unified Diagnostic Services) diagnostik arayüz çekirdeği.

🚀 Mevcut Özellikler
UDS İstek Oluşturucu (Request Builder): Temel UDS servis ID'lerini destekleyen nesne yönelimli paket oluşturma modülü:

0x10: Teşhis Oturum Kontrolü (Diagnostic Session Control)

0x22: Tanımlayıcıya Göre Veri Okuma (Read Data By Identifier)

0x19: DTC (Hata Kodu) Bilgisi Okuma

0x14: Teşhis Bilgilerini Temizleme (Clear Diagnostic Information)

Birim Testleri (Unit Testing): Güvenli çerçeve seri hale getirmeyi (serialization) garanti eden entegre test paketi.

🗺️ Yol Haritası (Roadmap)
[x] Temel UDS İstek Oluşturucu ve Birim Testleri

[ ] ISO-TP Segmentasyon Katmanı (iso-dep)

[ ] Linux socketcan / Sanal CAN (vcan0) entegrasyonu

[ ] ECU Simülasyonu (Motor ve Gövde ECU'ları)


