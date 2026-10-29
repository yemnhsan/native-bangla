; ============================================================
; Native Bangla IME — NSIS Installer Script
; Produces: NativeBangla-v{VERSION}-Setup.exe
; ============================================================

Unicode True

!define APP_NAME        "Native Bangla IME"
!define APP_PUBLISHER   "Native Bangla"
!define APP_URL         "https://github.com/yemnhsan/native-bangla"
!define INSTALL_DIR     "$PROGRAMFILES64\NativeBanglaIME"
!define REG_UNINST      "Software\Microsoft\Windows\CurrentVersion\Uninstall\NativeBanglaIME"
!define REG_ENV         "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"

!ifndef VERSION
  !define VERSION "0.1.0"
!endif

OutFile "..\..\dist\NativeBangla-v${VERSION}-Setup.exe"
Name    "${APP_NAME} ${VERSION}"

InstallDir              "${INSTALL_DIR}"
InstallDirRegKey HKLM   "${REG_UNINST}" "InstallLocation"
RequestExecutionLevel   admin
SetCompressor /SOLID    lzma
BrandingText            "${APP_NAME} ${VERSION} — github.com/yemnhsan/native-bangla"

; ── Modern UI ────────────────────────────────────────────────────────────────
!include "MUI2.nsh"

!define MUI_ABORTWARNING
!define MUI_ICON         "..\banner\icon.ico"
!define MUI_UNICON       "..\banner\icon.ico"
!define MUI_WELCOMEFINISHPAGE_BITMAP   "..\banner\welcome.bmp"
!define MUI_UNWELCOMEFINISHPAGE_BITMAP "..\banner\welcome.bmp"

!define MUI_WELCOMEPAGE_TITLE "Welcome to ${APP_NAME}"
!define MUI_WELCOMEPAGE_TEXT  "This wizard will install ${APP_NAME} ${VERSION} on your computer.$\r$\n$\r$\nNative Bangla IME is a fast, offline-first Bangla phonetic input method for Windows.$\r$\n$\r$\nClick Next to continue."

!define MUI_FINISHPAGE_TITLE  "Installation Complete!"
!define MUI_FINISHPAGE_TEXT   "${APP_NAME} is now installed.$\r$\n$\r$\nTo activate it:$\r$\n1. Open Windows Settings$\r$\n2. Go to Time & Language > Language$\r$\n3. Add 'Bengali (Bangladesh)'$\r$\n4. Select 'Native Bangla IME' as the input method$\r$\n$\r$\nYou may need to sign out and back in."
!define MUI_FINISHPAGE_LINK       "Open Language Settings"
!define MUI_FINISHPAGE_LINK_LOCATION "ms-settings:regionlanguage"

; Installer pages
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_LICENSE "..\..\LICENSE"
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

; Uninstaller pages
!insertmacro MUI_UNPAGE_WELCOME
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_UNPAGE_FINISH

!insertmacro MUI_LANGUAGE "English"

; ── Helper: Add dir to system PATH ───────────────────────────────────────────
!macro AddToPath DIR
  ; Read current PATH from registry
  ReadRegStr $0 HKLM "${REG_ENV}" "PATH"
  ; Only add if not already present
  ${StrContains} $1 "${DIR}" "$0"
  StrCmp $1 "" 0 +3
    WriteRegExpandStr HKLM "${REG_ENV}" "PATH" "$0;${DIR}"
    SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=500
!macroend

!macro RemoveFromPath DIR
  ReadRegStr $0 HKLM "${REG_ENV}" "PATH"
  ${StrStr} $1 "$0" ";${DIR}"
  StrCmp $1 "" 0 +2
    ${StrStr} $1 "$0" "${DIR};"
  StrCmp $1 "" done
    ; Simple remove — rebuild PATH without the entry
    ${StrRep} $2 "$0" ";${DIR}" ""
    ${StrRep} $2 "$2" "${DIR};" ""
    WriteRegExpandStr HKLM "${REG_ENV}" "PATH" "$2"
    SendMessage ${HWND_BROADCAST} ${WM_WININICHANGE} 0 "STR:Environment" /TIMEOUT=500
  done:
!macroend

!include "StrFunc.nsh"
${StrContains}
${StrStr}
${StrRep}

; ── Install ───────────────────────────────────────────────────────────────────
Section "Native Bangla IME" SecMain
  SectionIn RO

  SetOutPath "$INSTDIR"

  ; Core DLLs
  File "..\..\target\release\NativeBanglaIME.dll"
  File "..\..\target\release\ime_bridge.dll"
  File "..\..\target\release\libunwind.dll"

  ; Seed dictionary
  SetOutPath "$INSTDIR\resources\dictionaries"
  File "..\..\resources\dictionaries\bn-core-seed.json"

  SetOutPath "$INSTDIR"

  ; Write uninstaller
  WriteUninstaller "$INSTDIR\Uninstall.exe"

  ; Add install dir to system PATH so DLLs are found
  !insertmacro AddToPath "$INSTDIR"

  ; Register the IME with Windows Text Services Framework
  ExecWait 'regsvr32 /s "$INSTDIR\NativeBanglaIME.dll"'

  ; Add/Remove Programs entry
  WriteRegStr   HKLM "${REG_UNINST}" "DisplayName"          "${APP_NAME}"
  WriteRegStr   HKLM "${REG_UNINST}" "DisplayVersion"       "${VERSION}"
  WriteRegStr   HKLM "${REG_UNINST}" "Publisher"            "${APP_PUBLISHER}"
  WriteRegStr   HKLM "${REG_UNINST}" "URLInfoAbout"         "${APP_URL}"
  WriteRegStr   HKLM "${REG_UNINST}" "InstallLocation"      "$INSTDIR"
  WriteRegStr   HKLM "${REG_UNINST}" "UninstallString"      '"$INSTDIR\Uninstall.exe"'
  WriteRegStr   HKLM "${REG_UNINST}" "QuietUninstallString" '"$INSTDIR\Uninstall.exe" /S'
  WriteRegDWORD HKLM "${REG_UNINST}" "NoModify"             1
  WriteRegDWORD HKLM "${REG_UNINST}" "NoRepair"             1
  WriteRegDWORD HKLM "${REG_UNINST}" "EstimatedSize"        2500

  ; Start menu shortcut (points to uninstaller)
  CreateDirectory "$SMPROGRAMS\${APP_NAME}"
  CreateShortcut  "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk" "$INSTDIR\Uninstall.exe"

SectionEnd

; ── Uninstall ─────────────────────────────────────────────────────────────────
Section "Uninstall"

  ; Unregister the IME
  ExecWait 'regsvr32 /s /u "$INSTDIR\NativeBanglaIME.dll"'

  ; Remove from PATH
  !insertmacro RemoveFromPath "$INSTDIR"

  ; Delete files
  Delete "$INSTDIR\NativeBanglaIME.dll"
  Delete "$INSTDIR\ime_bridge.dll"
  Delete "$INSTDIR\libunwind.dll"
  Delete "$INSTDIR\resources\dictionaries\bn-core-seed.json"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir  "$INSTDIR\resources\dictionaries"
  RMDir  "$INSTDIR\resources"
  RMDir  "$INSTDIR"

  ; Start menu
  Delete "$SMPROGRAMS\${APP_NAME}\Uninstall.lnk"
  RMDir  "$SMPROGRAMS\${APP_NAME}"

  ; Registry
  DeleteRegKey HKLM "${REG_UNINST}"

SectionEnd
