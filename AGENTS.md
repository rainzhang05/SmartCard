# Project: PIV Management Platform

## Goal
Create a comprehensive platform for users to manage and modify PIV (Personal Identity Verification) data on their devices, including support for FIDO HID USB authenticators, tokens, and other CCID devices.

## Architecture
The project consists of three cooperating components:

1.  **Native Host (Rust)**
    -   **Role**: The "brain" of the operation. Handles all low-level smart card interactions (PC/SC), PIV logic, and device management.
    -   **Tech Stack**: Rust.
    -   **Responsibility**: Exposes a secure messaging interface to the extension, parses PIV objects, manages keys, and handles device connection/disconnection.

2.  **Web Platform (React)**
    -   **Role**: The user interface. Provides a premium, modern experience for managing devices.
    -   **Tech Stack**: React, Vite, TailwindCSS.
    -   **Responsibility**: Visualizes PIV data, guides users through management flows, and communicates with the Native Host via the Extension.

3.  **Browser Extension (Manifest V3)**
    -   **Role**: The bridge. Connects the Web Platform to the Native Host.
    -   **Tech Stack**: JavaScript/TypeScript, Manifest V3.
    -   **Responsibility**: Relays messages between the Web App and the Native Host using Native Messaging.

## Roadmap Overview
-   **Phase 1: Foundation**: Initialize Rust host, React app, and basic Extension structure.
-   **Phase 2: Core Logic**: Implement PIV standards and PC/SC communication in Rust.
-   **Phase 3: UI/UX**: Build the "wow" factor web interface.
-   **Phase 4: Integration**: Polish the communication flow (Web <-> Ext <-> Host).

## Key Directives
-   **Aesthetics Matter**: The web interface must be stunning, using modern design principles.
-   **Safety First**: Use Rust for the native host to ensure memory safety and reliability.
-   **Thick Host**: Move complex logic to the Rust host; keep the extension and web app as thin clients/views where possible.
