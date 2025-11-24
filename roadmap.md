# Project Roadmap: PIV Management Platform

## Phase 1: Foundation (Week 1)
- [ ] **Native Host**: Initialize a new Rust project (`cargo new`). Set up basic logging and Native Messaging loop.
- [ ] **Web Platform**: Initialize React + Vite + TailwindCSS project. Create basic layout and "Hello World" connection to extension.
- [ ] **Extension**: Update `manifest.json` to V3 (if not already fully compliant/optimized). Implement basic message passing between Web and Host.

## Phase 2: Core Logic (Weeks 2-3)
- [ ] **Smart Card Connection**: Implement PC/SC context and reader listing in Rust.
- [ ] **PIV Standards**: Implement ISO 7816-4 APDU command sending/receiving.
- [ ] **Data Parsing**: Implement parsing for PIV objects (CCC, CHUID, X.509 Certificates).
- [ ] **YubiKey/FIDO Support**: Add specific support for common devices (YubiKey, SoloKey, etc.).

## Phase 3: UI/UX (Weeks 4-5)
- [ ] **Dashboard**: Create a visual dashboard showing connected readers and inserted cards.
- [ ] **Certificate Manager**: UI for viewing, importing, and exporting certificates.
- [ ] **PIN Management**: Secure UI for changing PIN/PUK and management keys.
- [ ] **Design Polish**: Apply glassmorphism, animations, and premium styling.

## Phase 4: Integration & Polish (Week 6)
- [ ] **End-to-End Testing**: Verify full flow from Web -> Extension -> Host -> Card.
- [ ] **Cross-Platform**: Ensure `cargo build` works for macOS and Windows.
- [ ] **Packaging**: Create installers (`.pkg`, `.msi`) that bundle the Rust host and register the extension.
