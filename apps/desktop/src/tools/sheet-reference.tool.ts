// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/sheet-reference.tool.ts
// # 📌 Amac: Desktop Sheet grid koordinatlarini A1 referans metnine cevirir
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.4.0
// Aciklama: One-based UI grid koordinatlarini A1 referansina ve zero-based domain adresine deterministic map eder
// Bagimli Oldugu Katman: Tool

export interface SheetDomainAddress {
  readonly row: number;
  readonly column: number;
}

export class SheetReferenceTool {
  public columnLabel(column: number): string {
    let value = column;
    let label = "";
    while (value > 0) {
      const remainder = (value - 1) % 26;
      label = String.fromCharCode(65 + remainder) + label;
      value = Math.floor((value - 1) / 26);
    }
    return label;
  }

  public reference(row: number, column: number): string {
    return `${this.columnLabel(column)}${row}`;
  }

  public domainAddress(row: number, column: number): SheetDomainAddress {
    return {
      row: row - 1,
      column: column - 1,
    };
  }
}
