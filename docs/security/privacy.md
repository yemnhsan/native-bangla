# Privacy & Security Policy — Native Bangla

**Native Bangla — Type Naturally. Write Beautifully.**

Native Bangla is designed from the ground up as an offline-first, privacy-respecting input method editor.

## Core Privacy Commitments

### 1. 100% Offline Operation
Native Bangla never communicates with any remote server. There is no telemetry, no anonymous usage ping, no crash report uplink, and no cloud dictionary synchronization. The application has zero outbound network capability.

### 2. Secure Field Protection
When typing into password fields, PIN entries, or applications that set the TSF `TF_SS_TK_PASSWORD` or protected mode flag:
- Word prediction is immediately suspended.
- Next-word suggestions are completely hidden.
- Keystrokes and tokens are never added to the user's personal dictionary or frequency models.

### 3. Local-Only Storage
All user configuration and personalized vocabulary data are stored strictly on your local machine under the standard Windows roaming user profile:
`%APPDATA%\Native Bangla\preferences.json`

### 4. Data Control & Right to Erase
You retain absolute control over your input history:
- You can disable local learning at any time under **Settings → Dictionary → Local Learning**.
- You can clear your personalized word frequency models or reset all settings to default under **Settings → Advanced**.
- Uninstalling Native Bangla cleanly removes all registered COM classes, TSF profile keys, and local preference files.
