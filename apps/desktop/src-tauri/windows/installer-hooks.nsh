; # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/windows/installer-hooks.nsh
; # 📌 Amac: Windows Start Menu'de Turkuaz Office suite uygulamalarini ayri kisayollar olarak kurar
; # 📌 Modul - FileType: Distribution - NSIS
; Version: 0.5.0
; Aciklama: Generic Turkuaz Office kisayolunu Writer ve Sheet launch argumanli suite kisayollariyla degistirir
; Bagimli Oldugu Katman: CI | Distribution

!define TURKUAZ_SUITE_FOLDER "Turkuaz Office"
!define TURKUAZ_WRITER_SHORTCUT "Turkuaz Office Writer"
!define TURKUAZ_SHEET_SHORTCUT "Turkuaz Office Sheet"

!macro NSIS_HOOK_POSTINSTALL
  CreateDirectory "$SMPROGRAMS\${TURKUAZ_SUITE_FOLDER}"

  Delete "$SMPROGRAMS\${TURKUAZ_SUITE_FOLDER}\${PRODUCTNAME}.lnk"
  Delete "$SMPROGRAMS\${PRODUCTNAME}.lnk"
  Delete "$DESKTOP\${PRODUCTNAME}.lnk"

  CreateShortcut "$SMPROGRAMS\${TURKUAZ_SUITE_FOLDER}\${TURKUAZ_WRITER_SHORTCUT}.lnk" "$INSTDIR\${MAINBINARYNAME}.exe" "--module writer"
  CreateShortcut "$SMPROGRAMS\${TURKUAZ_SUITE_FOLDER}\${TURKUAZ_SHEET_SHORTCUT}.lnk" "$INSTDIR\${MAINBINARYNAME}.exe" "--module sheet"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  Delete "$SMPROGRAMS\${TURKUAZ_SUITE_FOLDER}\${TURKUAZ_WRITER_SHORTCUT}.lnk"
  Delete "$SMPROGRAMS\${TURKUAZ_SUITE_FOLDER}\${TURKUAZ_SHEET_SHORTCUT}.lnk"
  Delete "$SMPROGRAMS\${TURKUAZ_SUITE_FOLDER}\${PRODUCTNAME}.lnk"
  RMDir "$SMPROGRAMS\${TURKUAZ_SUITE_FOLDER}"
!macroend
