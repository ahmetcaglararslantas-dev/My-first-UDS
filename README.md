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