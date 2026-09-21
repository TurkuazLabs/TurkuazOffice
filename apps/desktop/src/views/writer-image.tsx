// # 📄 Dosya Yolu: E:/Projects/TurkuazOffice/apps/desktop/src/views/writer-image.tsx
// # 📌 Amac: Writer ImageBlock metadata'sini lazy asset fetch ile page yuzeyinde render eder
// # 📌 Modul - FileType: View - TSX
// # Version: 0.2.0
// # Aciklama: Binary asset'i Controller uzerinden ister, object URL yasam dongusunu cleanup ile kapatir
// Bagimli Oldugu Katman: View -> Controller

import { createSignal, onCleanup, onMount, Show } from "solid-js";

import type { WriterController } from "../controllers/writer.controller";
import type { WriterImageView } from "./writer-types";

interface WriterImageBlockProps {
  readonly documentId: string;
  readonly image: WriterImageView;
  readonly controller: WriterController;
}

export function WriterImageBlock(props: WriterImageBlockProps) {
  const [source, setSource] = createSignal<string | null>(null);
  let ownedUrl: string | null = null;

  onMount(() => {
    void props.controller
      .loadImageAssetUrl(props.documentId, props.image.assetId)
      .then((url) => {
        ownedUrl = url;
        setSource(url);
      })
      .catch(() => {
        setSource(null);
      });
  });

  onCleanup(() => {
    if (ownedUrl !== null) {
      props.controller.releaseImageAssetUrl(ownedUrl);
      ownedUrl = null;
    }
  });

  return (
    <figure class="writer-image-block" data-writer-image-id={props.image.id}>
      <Show
        when={source()}
        fallback={
          <span class="writer-image-block__placeholder">
            {props.image.altText || props.image.assetId}
          </span>
        }
      >
        {(url) => (
          <img
            class="writer-image-block__image"
            src={url()}
            alt={props.image.altText}
          />
        )}
      </Show>
    </figure>
  );
}
