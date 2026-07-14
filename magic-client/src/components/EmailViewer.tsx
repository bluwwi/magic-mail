"use client";

import * as api from "@/lib/api";

interface EmailViewerProps {
  email: api.Email;
  onClose: () => void;
  onDelete: (id: string) => void;
}

export default function EmailViewer({ email, onClose, onDelete }: EmailViewerProps) {
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
            className="text-sm text-white/80 leading-relaxed [&_a]:text-[#8D75E6] [&_a]:hover:underline [&_h1]:text-white [&_h2]:text-white [&_h3]:text-white [&_strong]:text-white [&_blockquote]:border-l-[#8D75E6] [&_blockquote]:pl-4 [&_blockquote]:border-l-2 [&_blockquote]:text-white/60 [&_code]:bg-white/10 [&_code]:px-1 [&_code]:rounded [&_pre]:bg-white/5 [&_pre]:p-4 [&_pre]:rounded-lg"
            dangerouslySetInnerHTML={{ __html: email.body_html }}
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
