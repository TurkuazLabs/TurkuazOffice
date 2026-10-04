; # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src-tauri/windows/hooks.nsh
; # 📌 Amac: Windows NSIS kurulumunda Turkuaz Writer ve Turkuaz Sheet icin ayri Start Menu kisayollari olusturur
; # 📌 Modul - FileType: Distribution - NSIS
; # Version: 0.4.0
; # Aciklama: Suite component kisayollarini launch module argumentlari ile olusturur ve uninstall sirasinda temizler
; # Bagimli Oldugu Katman: Distribution | Config

!macro NSIS_HOOK_POSTINSTALL
  CreateDirectory "$SMPROGRAMS\$AppStartMenuFolder"
  CreateShortcut "$SMPROGRAMS\$AppStartMenuFolder\Turkuaz Writer.lnk" "$INSTDIR\${MAINBINARYNAME}.exe" "--module=writer"
  CreateShortcut "$SMPROGRAMS\$AppStartMenuFolder\Turkuaz Sheet.lnk" "$INSTDIR\${MAINBINARYNAME}.exe" "--module=sheet"
!macroend

!macro NSIS_HOOK_PREUNINSTALL
  Delete "$SMPROGRAMS\$AppStartMenuFolder\Turkuaz Writer.lnk"
  Delete "$SMPROGRAMS\$AppStartMenuFolder\Turkuaz Sheet.lnk"
!macroend
