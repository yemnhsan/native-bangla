; ============================================================
; Native Bangla IME - Inno Setup Script
; Creates a standard Windows installer (.exe) for normal users
; ============================================================

#define MyAppName "Native Bangla IME"
#define MyAppVersion "0.1.0"
#define MyAppPublisher "Native Bangla Contributors"
#define MyAppURL "https://github.com/yemnhsan/native-bangla"

[Setup]
AppId={{8B2F3E4A-7C1D-4A5B-9E8F-0A1B2C3D4E5F}
AppName={#MyAppName}
AppVersion={#MyAppVersion}
AppPublisher={#MyAppPublisher}
AppPublisherURL={#MyAppURL}
AppSupportURL={#MyAppURL}/issues
AppUpdatesURL={#MyAppURL}/releases
DefaultDirName={autopf}\NativeBanglaIME
DefaultGroupName={#MyAppName}
DisableProgramGroupPage=yes
LicenseFile=..\..\LICENSE
OutputDir=..\..\dist
OutputBaseFilename=NativeBangla-v{#MyAppVersion}-Setup
SetupIconFile=..\banner\icon.ico
Compression=lzma2/ultra64
SolidCompression=yes
WizardStyle=modern
ArchitecturesAllowed=x64compatible
ArchitecturesInstallIn64BitMode=x64compatible
PrivilegesRequired=admin
ChangesEnvironment=yes
CloseApplications=no

[Languages]
Name: "english"; MessagesFile: "compiler:Default.isl"

[Files]
Source: "..\..\target\release\NativeBanglaIME.dll"; DestDir: "{app}"; Flags: 64bit restartreplace uninsrestartdelete
Source: "..\..\target\release\ime_bridge.dll"; DestDir: "{app}"; Flags: 64bit restartreplace uninsrestartdelete
Source: "..\..\target\release\libunwind.dll"; DestDir: "{app}"; Flags: 64bit restartreplace uninsrestartdelete
Source: "..\..\resources\dictionaries\bn-core-seed.json"; DestDir: "{app}\resources\dictionaries"; Flags: ignoreversion

[Registry]
Root: HKLM; Subkey: "SYSTEM\CurrentControlSet\Control\Session Manager\Environment"; \
  ValueType: expandsz; ValueName: "Path"; ValueData: "{olddata};{app}"; \
  Check: NeedsAddPath(ExpandConstant('{app}'))

[Run]
Filename: "{sys}\regsvr32.exe"; Parameters: "/s ""{app}\NativeBanglaIME.dll"""; WorkingDir: "{app}"; StatusMsg: "Registering Native Bangla IME with Windows..."; Flags: runhidden
Filename: "powershell.exe"; Parameters: "-ExecutionPolicy Bypass -NoProfile -Command ""$List = Get-WinUserLanguageList; if (-not ($List | Where-Object {{ $_.LanguageTag -like 'bn*' }})) {{ $List.Add('bn-BD'); Set-WinUserLanguageList $List -Force }}"""; Flags: runhidden; StatusMsg: "Enabling Bengali language in Windows..."
Filename: "ms-settings:regionlanguage"; Description: "Open Windows Language Settings"; Flags: postinstall shellexec skipifsilent unchecked

[UninstallRun]
Filename: "{sys}\regsvr32.exe"; Parameters: "/s /u ""{app}\NativeBanglaIME.dll"""; WorkingDir: "{app}"; Flags: runhidden; RunOnceId: "UnregisterNativeBanglaIME"

[Code]
function NeedsAddPath(Param: string): boolean;
var
  OrigPath: string;
begin
  if not RegQueryStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', OrigPath) then
  begin
    Result := True;
    exit;
  end;
  Result := Pos(';' + Uppercase(Param) + ';', ';' + Uppercase(OrigPath) + ';') = 0;
end;

procedure CurUninstallStepChanged(CurUninstallStep: TUninstallStep);
var
  OrigPath, AppDir: string;
begin
  if CurUninstallStep = usPostUninstall then
  begin
    AppDir := ExpandConstant('{app}');
    if RegQueryStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', OrigPath) then
    begin
      StringChangeEx(OrigPath, ';' + AppDir, '', True);
      StringChangeEx(OrigPath, AppDir + ';', '', True);
      StringChangeEx(OrigPath, AppDir, '', True);
      RegWriteStringValue(HKEY_LOCAL_MACHINE, 'SYSTEM\CurrentControlSet\Control\Session Manager\Environment', 'Path', OrigPath);
    end;
  end;
end;
