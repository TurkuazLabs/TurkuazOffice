// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/suite-icon.tsx
// # 📌 Amac: Turkuaz Office uygulama ailesi icin ayri fakat tutarli modul ikonlarini render eder
// # 📌 Modul - FileType: View - TSX
// Version: 0.12.0
// Aciklama: Brand, Home, Writer, Sheet, Sunum ve PDF icin ortak SVG ikon dilini tek View bileseninde tutar
// Bagimli Oldugu Katman: View

import { Match, Switch } from "solid-js";

export type SuiteIconKind =
  | "brand"
  | "home"
  | "writer"
  | "sheet"
  | "presentation"
  | "pdf";

interface SuiteIconProps {
  readonly kind: SuiteIconKind;
  readonly size?: number;
  readonly decorative?: boolean;
}

export function SuiteIcon(props: SuiteIconProps) {
  const size = () => props.size ?? 24;
  const label = () => (props.decorative === true ? undefined : props.kind);

  return (
    <svg
      class={`suite-icon suite-icon--${props.kind}`}
      width={size()}
      height={size()}
      viewBox="0 0 48 48"
      role={props.decorative === true ? "presentation" : "img"}
      aria-label={label()}
      aria-hidden={props.decorative === true ? "true" : undefined}
    >
      <rect class="suite-icon__tile" x="3" y="3" width="42" height="42" rx="11" />
      <Switch>
        <Match when={props.kind === "brand"}>
          <path class="suite-icon__mark" d="M13 14h22v7h-7v15h-8V21h-7z" />
          <rect class="suite-icon__detail" x="29" y="29" width="7" height="7" rx="2" />
        </Match>
        <Match when={props.kind === "home"}>
          <path class="suite-icon__mark" d="M10 24 24 12l14 12v13H28v-9h-8v9H10z" />
        </Match>
        <Match when={props.kind === "writer"}>
          <path class="suite-icon__page" d="M14 10h15l7 7v21H14z" />
          <path class="suite-icon__fold" d="M29 10v8h7" />
          <path class="suite-icon__line" d="M19 23h12M19 28h12M19 33h9" />
        </Match>
        <Match when={props.kind === "sheet"}>
          <rect class="suite-icon__page" x="13" y="10" width="22" height="28" rx="2" />
          <path class="suite-icon__line" d="M18 17h12M18 23h12M18 29h12M18 35h12M22 17v18M27 17v18" />
        </Match>
        <Match when={props.kind === "presentation"}>
          <rect class="suite-icon__page" x="12" y="12" width="24" height="22" rx="2" />
          <path class="suite-icon__line" d="M18 19h12M18 24h9M24 34v5M18 39h12" />
        </Match>
        <Match when={props.kind === "pdf"}>
          <path class="suite-icon__page" d="M14 10h15l7 7v21H14z" />
          <path class="suite-icon__fold" d="M29 10v8h7" />
          <path class="suite-icon__pdf" d="M18 31c5-8 7-12 8-12 1 5 3 9 7 12-6-2-10-2-15 0z" />
        </Match>
      </Switch>
    </svg>
  );
}
