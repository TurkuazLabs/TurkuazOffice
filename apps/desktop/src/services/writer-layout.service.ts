// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/services/writer-layout.service.ts
// # 📌 Amac: Writer canonical page geometry ile Desktop render geometry ve font fallback kararlarini koordine eder
// # 📌 Modul - FileType: Service - TypeScript
// # Version: 0.2.0
// # Aciklama: Twip -> CSS px, zoom clamp ve platform font resolution kurallarini View'dan ayirir
// Bagimli Oldugu Katman: Service -> Tool -> Config

import {
  CSS_REFERENCE_PIXELS_PER_INCH,
  TWIPS_PER_INCH,
  WRITER_DEFAULT_ZOOM_PERCENT,
  WRITER_FONT_FALLBACK_PROFILES,
  WRITER_GENERIC_FONT_FAMILY,
  WRITER_MAX_ZOOM_PERCENT,
  WRITER_MIN_ZOOM_PERCENT,
  WRITER_ZOOM_STEP_PERCENT,
  type WriterFontFallbackProfile,
} from "../config/layout";
import type { FontCapabilityTool } from "../tools/font-capability.tool";
import type {
  WriterDocumentView,
  WriterPageLayoutView,
  WriterPageSettingsView,
  WriterResolvedFontView,
} from "../views/writer-types";

export class WriterLayoutService {
  private readonly resolutionCache = new Map<string, WriterResolvedFontView>();

  public constructor(private readonly fontCapabilityTool: FontCapabilityTool) {}

  public zoomIn(currentPercent: number): number {
    return this.clampZoom(currentPercent + WRITER_ZOOM_STEP_PERCENT);
  }

  public zoomOut(currentPercent: number): number {
    return this.clampZoom(currentPercent - WRITER_ZOOM_STEP_PERCENT);
  }

  public resetZoom(): number {
    return WRITER_DEFAULT_ZOOM_PERCENT;
  }

  public pageLayout(settings: WriterPageSettingsView, zoomPercent: number): WriterPageLayoutView {
    const normalizedZoom = this.clampZoom(zoomPercent);
    const scale = normalizedZoom / 100;
    return {
      pageWidthPx: this.twipsToCssPixels(settings.widthTwips) * scale,
      pageHeightPx: this.twipsToCssPixels(settings.heightTwips) * scale,
      marginTopPx: this.twipsToCssPixels(settings.marginTopTwips) * scale,
      marginRightPx: this.twipsToCssPixels(settings.marginRightTwips) * scale,
      marginBottomPx: this.twipsToCssPixels(settings.marginBottomTwips) * scale,
      marginLeftPx: this.twipsToCssPixels(settings.marginLeftTwips) * scale,
      scale,
      zoomPercent: normalizedZoom,
    };
  }

  public resolveDocumentFonts(document: WriterDocumentView): readonly WriterResolvedFontView[] {
    const requestedFamilies = new Set<string>();
    for (const paragraph of document.paragraphs) {
      for (const run of paragraph.runs) {
        requestedFamilies.add(run.style.fontFamily);
      }
    }
    return [...requestedFamilies].map((family) => this.resolveFont(family));
  }

  public resolveFont(requestedFamily: string): WriterResolvedFontView {
    const cached = this.resolutionCache.get(requestedFamily);
    if (cached !== undefined) {
      return cached;
    }

    const profile = this.profileFor(requestedFamily);
    const candidates = this.unique([requestedFamily, ...profile.candidates]);
    const resolvedFamily = candidates.find((candidate) => this.fontCapabilityTool.isAvailable(candidate))
      ?? profile.genericFamily;
    const cssStack = [...candidates.map((candidate) => this.quoteCssFamily(candidate)), profile.genericFamily].join(", ");
    const result: WriterResolvedFontView = {
      requestedFamily,
      resolvedFamily,
      cssStack,
      substituted: resolvedFamily !== requestedFamily,
    };
    this.resolutionCache.set(requestedFamily, result);
    return result;
  }

  private profileFor(requestedFamily: string): WriterFontFallbackProfile {
    return WRITER_FONT_FALLBACK_PROFILES[requestedFamily] ?? {
      candidates: [],
      genericFamily: WRITER_GENERIC_FONT_FAMILY,
    };
  }

  private twipsToCssPixels(twips: number): number {
    return (twips / TWIPS_PER_INCH) * CSS_REFERENCE_PIXELS_PER_INCH;
  }

  private clampZoom(percent: number): number {
    return Math.min(WRITER_MAX_ZOOM_PERCENT, Math.max(WRITER_MIN_ZOOM_PERCENT, percent));
  }

  private unique(values: readonly string[]): string[] {
    return values.filter((value, index) => values.indexOf(value) === index);
  }

  private quoteCssFamily(fontFamily: string): string {
    const escaped = fontFamily.replaceAll("\\", "\\\\").replaceAll('"', '\\"');
    return `"${escaped}"`;
  }
}
