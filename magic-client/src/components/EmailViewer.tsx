"use client";

import { useEffect, useState } from "react";
import DOMPurify from "dompurify";
import * as api from "@/lib/api";

if (typeof window !== "undefined") {
  DOMPurify.addHook("afterSanitizeAttributes", (node) => {
    if (node.tagName === "IMG") {
      const img = node as HTMLImageElement;
      const src = img.getAttribute("src");
      if (src && src.startsWith("http://")) {
        img.setAttribute("src", src.replace(/^http:/, "https:"));
      }
      img.setAttribute("loading", "lazy");
      img.setAttribute("referrerpolicy", "no-referrer");
    }
  });
}

interface EmailViewerProps {
  email: api.Email;
  onClose: () => void;
  onDelete: (id: string) => void;
}

export default function EmailViewer({ email, onClose, onDelete }: EmailViewerProps) {
  const [sanitizedHtml, setSanitizedHtml] = useState("");

  useEffect(() => {
    if (!email.body_html || typeof window === "undefined") {
      setSanitizedHtml("");
      return;
    }
    const clean = DOMPurify.sanitize(email.body_html, {
      USE_PROFILES: { html: true },
      ALLOWED_TAGS: [
        "a", "abbr", "address", "article", "aside", "b", "bdi", "bdo",
        "blockquote", "br", "caption", "cite", "code", "col", "colgroup",
        "dd", "del", "details", "div", "dl", "dt", "em", "figcaption",
        "figure", "footer", "h1", "h2", "h3", "h4", "h5", "h6", "header",
        "hr", "i", "img", "ins", "kbd", "li", "main", "mark", "nav",
        "ol", "p", "pre", "q", "s", "samp", "section", "small", "span",
        "strong", "sub", "summary", "sup", "table", "tbody", "td",
        "tfoot", "th", "thead", "tr", "u", "ul", "var", "wbr",
      ],
      ALLOWED_ATTR: [
        "href", "src", "alt", "title", "width", "height", "style",
        "class", "id", "colspan", "rowspan", "target", "rel",
        "loading", "referrerpolicy", "border", "cellpadding", "cellspacing",
        "align", "valign", "bgcolor", "color",
      ],
    });
    setSanitizedHtml(clean);
  }, [email.body_html]);

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-start justify-between gap-4 pb-4 border-b border-white/10">
        <div className="flex-1 min-w-0">
          <h2 className="text-base font-semibold text-white truncate">
            {email.subject || "(No subject)"}
          </h2>
          <div className="mt-2 space-y-0.5">
            <p className="text-sm text-white/70">
              <span className="text-white/40">From:</span> {email.from_addr}
            </p>
            <p className="text-sm text-white/70">
              <span className="text-white/40">To:</span> {email.to_address}
            </p>
            <p className="text-xs text-white/40">
              {new Date(email.received_at * 1000).toLocaleString()}
            </p>
          </div>
        </div>
        <div className="flex items-center gap-2 flex-shrink-0">
          <button
            onClick={() => onDelete(email.id)}
            className="px-3 py-1.5 text-xs font-medium text-white/60 border border-white/20 rounded-lg hover:bg-white/10 transition-colors"
          >
            Delete
          </button>
          <button
            onClick={onClose}
            className="px-3 py-1.5 text-xs font-medium text-white/60 border border-white/20 rounded-lg hover:bg-white/10 transition-colors"
          >
            Close
          </button>
        </div>
      </div>

      <div className="flex-1 overflow-y-auto pt-4">
        {email.body_html ? (
          <div
            className="text-sm text-white/80 leading-relaxed [&_a]:text-[#8D75E6] [&_a]:hover:underline [&_h1]:text-white [&_h2]:text-white [&_h3]:text-white [&_strong]:text-white [&_blockquote]:border-l-[#8D75E6] [&_blockquote]:pl-4 [&_blockquote]:border-l-2 [&_blockquote]:text-white/60 [&_code]:bg-white/10 [&_code]:px-1 [&_code]:rounded [&_pre]:bg-white/5 [&_pre]:p-4 [&_pre]:rounded-lg [&_img]:max-w-full [&_img]:h-auto [&_img]:rounded-lg"
            dangerouslySetInnerHTML={{ __html: sanitizedHtml }}
          />
        ) : (
          <pre className="text-sm text-white/60 whitespace-pre-wrap font-sans leading-relaxed">
            {email.body_text || "(No content)"}
          </pre>
        )}
      </div>
    </div>
  );
}
