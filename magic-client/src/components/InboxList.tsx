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
      <div className="flex items-center justify-center h-full text-white/40 text-sm">
        Loading...
      </div>
    );
  }

  if (emails.length === 0) {
    return (
      <div className="flex items-center justify-center h-full text-white/40 text-sm px-4 text-center">
        No emails yet. Send a message to your temporary address.
      </div>
    );
  }

  return (
    <div className="h-full flex flex-col">
      <div className="flex items-center justify-between px-2 pb-2 border-b border-white/10">
        <span className="text-xs text-white/50">
          {emails.length} {emails.length === 1 ? "message" : "messages"}
        </span>
        <button
          onClick={onClear}
          className="text-xs text-white/40 hover:text-white/80 transition-colors"
        >
          Clear all
        </button>
      </div>
      <div className="flex-1 overflow-y-auto divide-y divide-white/10 mt-1">
        {emails.map((email) => (
          <div
            key={email.id}
            role="button"
            tabIndex={0}
            aria-label={`Email: ${email.subject || "(No subject)"} from ${email.from_addr}`}
            onClick={() => onSelect(email)}
            onKeyDown={(e) => {
              if (e.key === "Enter" || e.key === " ") {
                e.preventDefault();
                onSelect(email);
              }
            }}
            className={`px-2 py-3 cursor-pointer transition-colors focus:outline-none focus-visible:ring-2 focus-visible:ring-white/30 ${
              selectedId === email.id
                ? "bg-white/10 border-l-2 border-l-white"
                : "hover:bg-white/5 border-l-2 border-l-transparent"
            }`}
          >
            <div className="flex items-start justify-between gap-2">
              <div className="flex-1 min-w-0">
                <div className="flex items-center gap-2">
                  {!email.is_read && (
                    <span className="w-2 h-2 rounded-full bg-white flex-shrink-0" />
                  )}
                  <p
                    className={`text-sm truncate ${
                      email.is_read
                        ? "text-white/60"
                        : "text-white font-medium"
                    }`}
                  >
                    {email.subject || "(No subject)"}
                  </p>
                </div>
                <p className="text-xs text-white/40 truncate mt-0.5 ml-4">
                  {email.from_addr}
                </p>
                <p className="text-[11px] text-white/30 mt-1 ml-4">
                  {new Date(email.received_at * 1000).toLocaleString()}
                </p>
              </div>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onDelete(email.id);
                }}
                className="flex-shrink-0 p-1 rounded text-white/30 hover:text-white/80 hover:bg-white/10 transition-colors"
                title="Delete"
              >
                <svg
                  className="w-3.5 h-3.5"
                  fill="none"
                  stroke="currentColor"
                  viewBox="0 0 24 24"
                >
                  <path
                    strokeLinecap="round"
                    strokeLinejoin="round"
                    strokeWidth={2}
                    d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
                  />
                </svg>
              </button>
            </div>
          </div>
        ))}
      </div>
    </div>
  );
}
