"use client";

import * as api from "@/lib/api";

interface InboxListProps {
  emails: api.Email[];
  selectedId: string | null;
  loading: boolean;
  onSelect: (email: api.Email) => void;
  onDelete: (id: string) => void;
  onClear: () => void;
}

export default function InboxList({
  emails,
  selectedId,
  loading,
  onSelect,
  onDelete,
  onClear,
}: InboxListProps) {
  if (loading && emails.length === 0) {
    return (
      <div className="bg-[var(--bg-card)] rounded-xl border border-[var(--border)] p-8 flex items-center justify-center">
        <div className="flex items-center gap-2 text-[var(--text-muted)] text-sm">
          <span className="w-4 h-4 border-2 border-[var(--accent)] border-t-transparent rounded-full animate-spin" />
          Loading...
        </div>
      </div>
    );
  }

  if (emails.length === 0) {
    return (
      <div className="bg-[var(--bg-card)] rounded-xl border border-[var(--border)] p-12 text-center">
        <div className="w-12 h-12 mx-auto mb-4 rounded-full bg-[var(--bg-secondary)] flex items-center justify-center">
          <svg className="w-6 h-6 text-[var(--text-muted)]" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={1.5} d="M3 8l7.89 5.26a2 2 0 002.22 0L21 8M5 19h14a2 2 0 002-2V7a2 2 0 00-2-2H5a2 2 0 00-2 2v10a2 2 0 002 2z" />
          </svg>
        </div>
        <p className="text-[var(--text-secondary)] text-sm font-medium">No emails yet</p>
        <p className="text-[var(--text-muted)] text-xs mt-1.5">
          Emails sent to your temporary address will appear here in real-time.
        </p>
      </div>
    );
  }

  return (
    <div className="bg-[var(--bg-card)] rounded-xl border border-[var(--border)] overflow-hidden">
      <div className="flex items-center justify-between px-4 py-2.5 border-b border-[var(--border)] bg-[var(--bg-secondary)]">
        <span className="text-sm text-[var(--text-secondary)]">
          {emails.length} {emails.length === 1 ? "message" : "messages"}
        </span>
        <button
          onClick={onClear}
          className="text-xs text-[var(--text-muted)] hover:text-[var(--danger)] transition-colors duration-200"
        >
          Clear all
        </button>
      </div>

      <div className="divide-y divide-[var(--border)] max-h-[480px] overflow-y-auto scrollbar-thin">
        {emails.map((email) => (
          <div
            key={email.id}
            onClick={() => onSelect(email)}
            className={`px-4 py-3 cursor-pointer transition-all duration-200 ${
              selectedId === email.id
                ? "bg-[var(--accent-glow)] border-l-2 border-l-[var(--accent)]"
                : "hover:bg-[var(--bg-card-hover)] border-l-2 border-l-transparent"
            }`}
          >
            <div className="flex items-start justify-between gap-2">
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  {!email.is_read && (
                    <span className="w-2 h-2 rounded-full bg-[var(--accent)] flex-shrink-0 animate-pulse" />
                  )}
                  <p className={`text-sm truncate ${
                    email.is_read ? "text-[var(--text-secondary)]" : "text-[var(--text-primary)] font-medium"
                  }`}>
                    {email.subject || "(No subject)"}
                  </p>
                </div>
                <p className="text-xs text-[var(--text-muted)] truncate mt-0.5 ml-4">
                  {email.from_address}
                </p>
                <p className="text-[11px] text-[var(--text-muted)] mt-1 ml-4">
                  {new Date(email.created_at * 1000).toLocaleString()}
                </p>
              </div>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onDelete(email.id);
                }}
                className="flex-shrink-0 p-1.5 rounded-lg text-[var(--text-muted)] hover:text-[var(--danger)] hover:bg-[var(--danger-bg)] transition-all duration-200 opacity-0 group-hover:opacity-100"
                title="Delete"
              >
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
