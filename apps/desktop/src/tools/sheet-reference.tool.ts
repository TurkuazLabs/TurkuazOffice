// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/tools/sheet-reference.tool.ts
// # 📌 Amac: Desktop Sheet grid koordinatlarini A1 referans metnine cevirir
// # 📌 Modul - FileType: Tool - TypeScript
// Version: 0.5.0
// Aciklama: One-based UI grid koordinatlarini A1 referansina ve zero-based domain adresine deterministic map eder
// Bagimli Oldugu Katman: Tool

export interface SheetDomainAddress {
  readonly row: number;
  readonly column: number;
}

export type SheetGridNavigationDirection = "up" | "down" | "left" | "right";

export interface SheetGridNavigationTarget {
  readonly row: number;
  readonly column: number;
  readonly reference: string;
  readonly address: SheetDomainAddress;
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

  public adjacentGridCell(
    row: number,
    column: number,
    direction: SheetGridNavigationDirection,
    rowCount: number,
    columnCount: number,
  ): SheetGridNavigationTarget {
    let targetRow = row;
    let targetColumn = column;

    switch (direction) {
      case "up":
        targetRow = Math.max(1, row - 1);
        break;
      case "down":
        targetRow = Math.min(rowCount, row + 1);
        break;
      case "right":
        if (column < columnCount) {
          targetColumn = column + 1;
        } else if (row < rowCount) {
          targetRow = row + 1;
          targetColumn = 1;
        }
        break;
      case "left":
        if (column > 1) {
          targetColumn = column - 1;
        } else if (row > 1) {
          targetRow = row - 1;
          targetColumn = columnCount;
        }
        break;
    }

    return {
      row: targetRow,
      column: targetColumn,
      reference: this.reference(targetRow, targetColumn),
      address: this.domainAddress(targetRow, targetColumn),
    };
  }
}
