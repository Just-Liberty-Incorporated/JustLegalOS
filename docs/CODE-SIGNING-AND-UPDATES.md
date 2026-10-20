# Code Signing & Auto-Updater Runbook

This guide details how to configure code signing certificates and signed auto-updates for **JustLegal OS** before public scale.

---

## 1. Why Code Signing & Auto-Updates Matter

At launch scale:
- **Windows SmartScreen:** Unsigned `.exe` and `.msi` installers show a blue "Windows protected your PC — Unknown publisher" banner that deters non-technical users. A signed binary with an EV certificate or Azure Trusted Signing establishes instant reputation.
- **macOS Gatekeeper:** Modern macOS versions (Ventura, Sonoma, Sequoia) will block unsigned `.dmg` or `.app` bundles entirely with "app is damaged and cannot be opened" or "cannot verify developer". Apple Notarization is mandatory for seamless double-click installation on Mac.
- **Auto-Updates:** Legal rules, procedural deadlines (e.g. NJ R. 2:11-6 or Colorado C.R.C.P. 121), and court form registries change. The Tauri auto-updater ensures users and bots receive deterministic rule corrections without manually reinstalling the application.

---

## 2. Setting Up the Tauri Auto-Updater

Tauri v2 uses cryptographic ed25519 signatures to ensure that update packages cannot be tampered with between your release build and user computers.

### Step 2.1: Generate the Keypair

Run the following command in your terminal:
```bash
npx @tauri-apps/cli signer generate -w ~/.tauri/justlegal.key
```
This produces two keys:
1. **Public Key:** Printed to stdout (e.g. `dW50cnVzdGVkIGNvbW1lbnQ6IG1pbmlzaWduIHB1YmxpYyBrZXk...`).
2. **Private Key:** Saved to `~/.tauri/justlegal.key`.

### Step 2.2: Add the Public Key to `tauri.conf.json`

Open `src-tauri/tauri.conf.json` and insert your public key string:
```json
"plugins": {
  "updater": {
    "endpoints": [
      "https://github.com/Just-Liberty-Incorporated/JustLegalOS/releases/latest/download/latest.json",
      "https://github.com/unlimitedinfinit/JustLegalOS/releases/latest/download/latest.json",
      "https://github.com/unlimitedinfinit/OpenSeason/releases/latest/download/latest.json"
    ],
    "pubkey": "PASTE_YOUR_PUBLIC_KEY_STRING_HERE"
  }
}
```

### Step 2.3: Add Private Key to GitHub Repository Secrets

In your GitHub repository, navigate to **Settings > Secrets and variables > Actions** and add:
- `TAURI_SIGNING_PRIVATE_KEY`: Content of `~/.tauri/justlegal.key`.
- `TAURI_SIGNING_PRIVATE_KEY_PASSWORD`: (Optional) The password you set when generating the key, if any.

When a tag (e.g. `v0.1.0`) is pushed, the `.github/workflows/release.yml` pipeline will automatically sign the release bundles and upload `latest.json`.

---

## 3. macOS Code Signing & Apple Notarization

To ship zero-warning `.dmg` and `.app` bundles on macOS:

### Step 3.1: Prerequisites
- Apple Developer Program account ($99/year) from [developer.apple.com](https://developer.apple.com).
- Xcode on macOS (or Keychain Access) to generate a **Developer ID Application** certificate.

### Step 3.2: Export the Certificate as `.p12`
1. In Keychain Access, find your **Developer ID Application: Your Name / Just Liberty Inc (TEAM_ID)** certificate.
2. Right-click > Export > save as `DeveloperIDApp.p12` with a password.
3. Base64-encode the file:
   ```bash
   base64 -i DeveloperIDApp.p12 | pbcopy
   ```

### Step 3.3: Configure GitHub Repository Secrets
Add the following secrets to GitHub:
- `APPLE_CERTIFICATE`: The base64-encoded `.p12` string.
- `APPLE_CERTIFICATE_PASSWORD`: The password chosen during export.
- `APPLE_SIGNING_IDENTITY`: The exact string name, e.g. `Developer ID Application: Just Liberty Inc (TEAM_ID)`.
- `APPLE_ID`: Your Apple ID email address.
- `APPLE_PASSWORD`: An App-Specific Password generated at [appleid.apple.com](https://appleid.apple.com) under Security.
- `APPLE_TEAM_ID`: Your 10-character Apple Team ID (found in Developer Account membership).

`tauri-action` in `.github/workflows/release.yml` will automatically sign and notarize the macOS `.dmg` and `.app` bundles when these secrets are present.

---

## 4. Windows Code Signing (SmartScreen Reputation)

Windows SmartScreen requires code signing to eliminate "Unknown Publisher" warnings:

### Option A: Azure Trusted Signing (Recommended for Nonprofits & Modern Teams)
Microsoft Azure offers **Azure Trusted Signing** (formerly Identity Verification / Trusted Signing), which replaces expensive ($400+/year) physical USB HSM hardware tokens with a cloud-managed certificate.
1. Create a Microsoft Azure account for Just Liberty Inc.
2. Search for **Trusted Signing Accounts** in Azure Portal.
3. Complete identity verification as a 501(c)(3) organization.
4. Integrate with GitHub Actions using the official `actions/trusted-signing-action` or configuring Azure SignTool.

### Option B: Traditional OV/EV Code Signing Certificate
1. Purchase a standard OV (Organization Validation) or EV (Extended Validation) code-signing certificate from DigiCert or Sectigo.
2. Configure `certificateThumbprint` in `src-tauri/tauri.conf.json` under `bundle.windows`:
   ```json
   "windows": {
     "certificateThumbprint": "YOUR_THUMBPRINT_HEX",
     "digestAlgorithm": "sha256",
     "timestampUrl": "http://timestamp.digicert.com"
   }
   ```

---

## 5. Graceful CI Degradation

The `.github/workflows/release.yml` workflow is designed with **graceful degradation**:
- If signing secrets are **not yet set**, the CI build still completes successfully and produces standard, functional installers and CLI binaries.
- Once secrets are **configured**, signing and notarization activate automatically on tag pushes without code changes.
