Unicode true
!include "MUI2.nsh"
!include "FileFunc.nsh"
!include "LogicLib.nsh"
!ifndef VERSION
!define VERSION "1.2.0-preview.8"
!endif
!ifndef APP_EXE
!define APP_EXE "..\target\release\rs7-isik.exe"
!endif
!ifndef OUTPUT
!define OUTPUT "..\RS7-Light-v${VERSION}-Setup-x64.exe"
!endif
Name "RS7 Light"
OutFile "${OUTPUT}"
InstallDir "$LOCALAPPDATA\Programs\RS7 Light"
RequestExecutionLevel user
SetCompressor /SOLID lzma
BrandingText "RS7 Light"
!ifndef VERSION_QUAD
!define VERSION_QUAD "1.2.0.0"
!endif
VIProductVersion "${VERSION_QUAD}"
VIAddVersionKey /LANG=1055 "ProductName" "RS7 Light"
VIAddVersionKey /LANG=1055 "FileDescription" "RS7 Light Kurulumu"
VIAddVersionKey /LANG=1055 "FileVersion" "${VERSION}"
VIAddVersionKey /LANG=1055 "ProductVersion" "${VERSION}"
VIAddVersionKey /LANG=1055 "LegalCopyright" "2026 tahayasinaks · MIT"
!define MUI_ICON "..\assets\rs7.ico"
!define MUI_UNICON "..\assets\rs7.ico"
!define MUI_ABORTWARNING
!define MUI_FINISHPAGE_RUN
!define MUI_FINISHPAGE_RUN_FUNCTION LaunchApp
!define MUI_FINISHPAGE_RUN_TEXT "RS7 Light uygulamasını aç"
!insertmacro MUI_PAGE_WELCOME
!insertmacro MUI_PAGE_DIRECTORY
!insertmacro MUI_PAGE_INSTFILES
!insertmacro MUI_PAGE_FINISH
!insertmacro MUI_UNPAGE_CONFIRM
!insertmacro MUI_UNPAGE_INSTFILES
!insertmacro MUI_LANGUAGE "Turkish"
Var TestMode
Var ExistingStartup
Function .onInit
  SetShellVarContext current
  ${GetParameters} $0
  ClearErrors
  ${GetOptions} $0 "/TESTMODE" $1
  IfErrors normal
  StrCpy $TestMode "1"
  Return
normal:
  ReadRegStr $1 HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\RS7Light" "InstallLocation"
  StrCmp $1 "" +2
  StrCpy $INSTDIR $1
  ClearErrors
  ${GetOptions} $0 "/UPDATEPID=" $1
  IfErrors check_running
  System::Call 'kernel32::OpenProcess(i 0x100000, i 0, i r1) p.r2'
  StrCmp $2 0 check_running
  System::Call 'kernel32::WaitForSingleObject(p r2, i 30000) i.r3'
  System::Call 'kernel32::CloseHandle(p r2)'
  StrCmp $3 258 wait_error check_running
wait_error:
  MessageBox MB_OK|MB_ICONEXCLAMATION "RS7 Light kapanmadı. Sistem tepsisinden Tamamen çık seçip kurulumu tekrar açın."
  Abort
check_running:
  System::Call 'kernel32::OpenMutexW(i 0x100000, i 0, w "Local\RS7IsikRust.SingleInstance") p.r2'
  StrCmp $2 0 done
  System::Call 'kernel32::CloseHandle(p r2)'
  MessageBox MB_RETRYCANCEL|MB_ICONINFORMATION "Kurulumdan önce RS7 uygulamasını sistem tepsisi menüsünden Tamamen çık ile kapatın." IDRETRY check_running
  Abort
done:
FunctionEnd
Function LaunchApp
  StrCmp $TestMode "1" +2
  Exec '"$INSTDIR\RS7 Light.exe"'
FunctionEnd
Section "RS7 Light" Main
  SetOutPath "$INSTDIR"
  ClearErrors
  File "/oname=RS7 Light.exe" "${APP_EXE}"
  IfErrors install_error
  File /oname=LICENSE.txt "..\LICENSE"
  WriteUninstaller "$INSTDIR\Uninstall.exe"
  StrCmp $TestMode "1" finish
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\RS7Light" "DisplayName" "RS7 Light"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\RS7Light" "DisplayVersion" "${VERSION}"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\RS7Light" "Publisher" "tahayasinaks"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\RS7Light" "InstallLocation" "$INSTDIR"
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\RS7Light" "UninstallString" '$\"$INSTDIR\Uninstall.exe$\"'
  WriteRegDWORD HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\RS7Light" "NoModify" 1
  WriteRegDWORD HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\RS7Light" "NoRepair" 1
  CreateDirectory "$SMPROGRAMS\RS7 Light"
  CreateShortcut "$SMPROGRAMS\RS7 Light\RS7 Light.lnk" "$INSTDIR\RS7 Light.exe"
  CreateShortcut "$DESKTOP\RS7 Light.lnk" "$INSTDIR\RS7 Light.exe"
  ReadRegStr $ExistingStartup HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "RS7"
  StrCmp $ExistingStartup "" finish
  WriteRegStr HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "RS7" '$\"$INSTDIR\RS7 Light.exe$\" --autostart'
  Goto finish
install_error:
  MessageBox MB_OK|MB_ICONSTOP "Uygulama dosyası yazılamadı. RS7 Light'ı kapatıp tekrar deneyin."
  Abort
finish:
SectionEnd
Function un.onInit
  SetShellVarContext current
  ${GetParameters} $0
  ClearErrors
  ${GetOptions} $0 "/TESTMODE" $1
  IfErrors +2
  StrCpy $TestMode "1"
FunctionEnd
Section "Uninstall"
  StrCmp $TestMode "1" files
  System::Call 'kernel32::OpenMutexW(i 0x100000, i 0, w "Local\RS7IsikRust.SingleInstance") p.r2'
  StrCmp $2 0 remove
  System::Call 'kernel32::CloseHandle(p r2)'
  MessageBox MB_OK|MB_ICONEXCLAMATION "RS7 Light'ı sistem tepsisi menüsünden Tamamen çık ile kapatıp kaldırmayı tekrar deneyin."
  Abort
remove:
  ReadRegStr $0 HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "RS7"
  StrCmp $0 '$\"$INSTDIR\RS7 Light.exe$\" --autostart' 0 +2
  DeleteRegValue HKCU "Software\Microsoft\Windows\CurrentVersion\Run" "RS7"
  DeleteRegKey HKCU "Software\Microsoft\Windows\CurrentVersion\Uninstall\RS7Light"
  Delete "$DESKTOP\RS7 Light.lnk"
  Delete "$SMPROGRAMS\RS7 Light\RS7 Light.lnk"
  RMDir "$SMPROGRAMS\RS7 Light"
files:
  Delete "$INSTDIR\RS7 Light.exe"
  Delete "$INSTDIR\LICENSE.txt"
  Delete "$INSTDIR\Uninstall.exe"
  RMDir "$INSTDIR"
  ; User's lighting/window settings are deliberately preserved.
SectionEnd

