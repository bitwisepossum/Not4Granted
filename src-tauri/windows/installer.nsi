Unicode true
ManifestDPIAware true
ManifestDPIAwareness PerMonitorV2

; Minimal Tauri-compatible NSIS template

{{#if signed_plugins_path}}
!addplugindir "{{signed_plugins_path}}"
{{/if}}

!include "MUI2.nsh"
!include "x64.nsh"
!include "FileFunc.nsh"

!define PRODUCTNAME "{{product_name}}"
!define VERSION "{{version}}"
!define VERSIONWITHBUILD "{{version_with_build}}"
!define MANUFACTURER "{{manufacturer}}"
!define COPYRIGHT "{{copyright}}"
!define MAINBINARYNAME "{{main_binary_name}}"
!define MAINBINARYSRCPATH "{{main_binary_path}}"
!define BUNDLEID "{{bundle_id}}"
!define OUTFILE "{{out_file}}"
!define INSTALLERICON "{{installer_icon}}"
!define LICENSEFILE "{{license}}"

!define UNINSTALLKEY "Software\Microsoft\Windows\CurrentVersion\Uninstall\${PRODUCTNAME}"
!define PRODUCTKEY "Software\${MANUFACTURER}\${PRODUCTNAME}"

Name "${PRODUCTNAME}"
OutFile "${OUTFILE}"
InstallDir "$LOCALAPPDATA\${PRODUCTNAME}"
InstallDirRegKey HKCU "${PRODUCTKEY}" "InstallLocation"
RequestExecutionLevel user
SetCompressor /SOLID lzma
ShowInstDetails show
ShowUninstDetails show

VIProductVersion "${VERSIONWITHBUILD}"
VIAddVersionKey "ProductName" "${PRODUCTNAME}"
VIAddVersionKey "FileDescription" "${PRODUCTNAME} installer"
VIAddVersionKey "FileVersion" "${VERSION}"
VIAddVersionKey "ProductVersion" "${VERSION}"
VIAddVersionKey "LegalCopyright" "${COPYRIGHT}"

!if "${INSTALLERICON}" != ""
  Icon "${INSTALLERICON}"
  !define MUI_ICON "${INSTALLERICON}"
!endif

!define MUI_ABORTWARNING
!define MUI_FINISHPAGE_RUN "$INSTDIR\${MAINBINARYNAME}.exe"

!insertmacro MUI_PAGE_WELCOME

!if "${LICENSEFILE}" != ""
  !insertmacro MUI_PAGE_LICENSE "${LICENSEFILE}"
!endif

!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH

!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES

{{#each languages}}
!insertmacro MUI_LANGUAGE "{{this}}"
{{/each}}

Function .onInit
  SetShellVarContext current

  ${If} ${RunningX64}
    SetRegView 64
  ${EndIf}
FunctionEnd

Section "Install" SEC_INSTALL
  ; A fresh log proves that this section actually started.
  FileOpen $9 "$TEMP\n4g-minimal-nsis.log" w
  FileWrite $9 "INSTALL SECTION STARTED$\r$\n"
  FileWrite $9 "INSTDIR=$INSTDIR$\r$\n"
  FileClose $9

  SetOutPath "$INSTDIR"
  SetOverwrite on

  ; Main Tauri executable. The frontend is embedded in this binary.
  File "${MAINBINARYSRCPATH}"

  ; Resources declared through bundle.resources.
  {{#each resources_dirs}}
  CreateDirectory "$INSTDIR\{{this}}"
  {{/each}}
  {{#each resources}}
  File /a "/oname={{this.[1]}}" "{{no-escape @key}}"
  {{/each}}

  ; Sidecars declared through bundle.externalBin.
  {{#each binaries}}
  File /a "/oname={{this}}" "{{no-escape @key}}"
  {{/each}}

  WriteUninstaller "$INSTDIR\uninstall.exe"

  WriteRegStr HKCU "${PRODUCTKEY}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "${UNINSTALLKEY}" "DisplayName" "${PRODUCTNAME}"
  WriteRegStr HKCU "${UNINSTALLKEY}" "DisplayVersion" "${VERSION}"
  WriteRegStr HKCU "${UNINSTALLKEY}" "Publisher" "${MANUFACTURER}"
  WriteRegStr HKCU "${UNINSTALLKEY}" "DisplayIcon" '$"$INSTDIR\${MAINBINARYNAME}.exe$"'
  WriteRegStr HKCU "${UNINSTALLKEY}" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "${UNINSTALLKEY}" "UninstallString" '$"$INSTDIR\uninstall.exe$"'
  WriteRegDWORD HKCU "${UNINSTALLKEY}" "NoModify" 1
  WriteRegDWORD HKCU "${UNINSTALLKEY}" "NoRepair" 1

  ${GetSize} "$INSTDIR" "/S=0K" $0 $1 $2
  IntFmt $0 "0x%08X" $0
  WriteRegDWORD HKCU "${UNINSTALLKEY}" "EstimatedSize" "$0"

  CreateShortcut "$SMPROGRAMS\${PRODUCTNAME}.lnk" "$INSTDIR\${MAINBINARYNAME}.exe"

  FileOpen $9 "$TEMP\n4g-minimal-nsis.log" a
  FileWrite $9 "INSTALL SECTION COMPLETED$\r$\n"
  FileClose $9
SectionEnd

Section "Uninstall"
  SetShellVarContext current

  ${If} ${RunningX64}
    SetRegView 64
  ${EndIf}

  Delete "$SMPROGRAMS\${PRODUCTNAME}.lnk"
  Delete "$INSTDIR\${MAINBINARYNAME}.exe"

  {{#each resources}}
  Delete "$INSTDIR\{{this.[1]}}"
  {{/each}}
  {{#each binaries}}
  Delete "$INSTDIR\{{this}}"
  {{/each}}

  Delete "$INSTDIR\uninstall.exe"

  {{#each resources_dirs}}
  RMDir "$INSTDIR\{{this}}"
  {{/each}}
  RMDir "$INSTDIR"

  DeleteRegKey HKCU "${UNINSTALLKEY}"
  DeleteRegKey HKCU "${PRODUCTKEY}"
SectionEnd
