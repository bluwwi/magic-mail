"use client";

import * as api from "@/lib/api";

interface EmailViewerProps {
  email: api.Email;
  onClose: () => void;
  onDelete: (id: string) => void;
}

export default function EmailViewer({ email, onClose, onDelete }: EmailViewerProps) {
  return (
    <div className="bg-[var(--bg-card)] rounded-xl border border-[var(--border)] overflow-hidden animate-slide-in">
      <div className="px-5 py-4 border-b border-[var(--border)]">
        <div className="flex items-start justify-between gap-4">
          <div className="flex-1 min-w-0">
            <h2 className="text-base font-semibold text-[var(--text-primary)] truncate">
              {email.subject || "(No subject)"}
            </h2>
            <div className="mt-2 space-y-0.5">
              <p className="text-sm text-[var(--text-secondary)]">
                <span className="text-[var(--text-muted)]">From:</span> {email.from_address}
              </p>
              <p className="text-sm text-[var(--text-secondary)]">
                <span className="text-[var(--text-muted)]">To:</span> {email.to_address}
              </p>
              <p className="text-xs text-[var(--text-muted)]">
                {new Date(email.created_at * 1000).toLocaleString()}
              </p>
            </div>
          </div>
          <div className="flex items-center gap-2 flex-shrink-0">
            <button
              onClick={() => onDelete(email.id)}
              className="px-3 py-1.5 text-xs font-medium text-[var(--danger)] border border-[var(--danger)]/30 rounded-lg hover:bg-[var(--danger-bg)] transition-all duration-200"
            >
              Delete
            </button>
            <button
              onClick={onClose}
              className="px-3 py-1.5 text-xs font-medium text-[var(--text-secondary)] border border-[var(--border)] rounded-lg hover:bg-[var(--bg-card-hover)] transition-all duration-200"
            >
              Close
            </button>
          </div>
        </div>
      </div>

      <div className="px-5 py-4 max-h-[520px] overflow-y-auto scrollbar-thin">
        {email.body_html ? (
          <div
            className="prose prose-sm max-w-none text-[var(--text-primary)] [&_a]:text-[var(--accent)] [&_a]:hover:text-[var(--accent-hover)] [&_h1]:text-[var(--text-primary)] [&_h2]:text-[var(--text-primary)] [&_h3]:text-[var(--text-primary)] [&_strong]:text-[var(--text-primary)] [&_blockquote]:border-l-[var(--accent)] [&_code]:bg-[var(--bg-secondary)] [&_pre]:bg-[var(--bg-secondary)]"
            dangerouslySetInnerHTML={{ __html: email.body_html }}
          />
        ) : (
          <pre className="text-sm text-[var(--text-secondary)] whitespace-pre-wrap font-sans leading-relaxed">
            {email.body_text || "(No content)"}
          </pre>
        )}
      </div>
    </div>
  );
}
