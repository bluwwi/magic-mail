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
      <div className="flex items-center justify-center py-12 text-gray-500">
        Loading...
      </div>
    );
  }

  if (emails.length === 0) {
    return (
      <div className="text-center py-12">
        <p className="text-gray-500 mb-2">No emails yet</p>
        <p className="text-sm text-gray-400">
          Emails sent to your temporary address will appear here in real-time.
        </p>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200">
      <div className="flex items-center justify-between px-4 py-2 border-b border-gray-200 bg-gray-50 rounded-t-lg">
        <span className="text-sm text-gray-600">
          {emails.length} {emails.length === 1 ? "email" : "emails"}
        </span>
        <button
          onClick={onClear}
          className="text-sm text-red-600 hover:text-red-800 transition-colors"
        >
          Clear all
        </button>
      </div>

      <ul className="divide-y divide-gray-200">
        {emails.map((email) => (
          <li
            key={email.id}
            onClick={() => onSelect(email)}
            className={`px-4 py-3 cursor-pointer hover:bg-gray-50 transition-colors animate-slide-in ${
              selectedId === email.id ? "bg-blue-50" : ""
            }`}
          >
            <div className="flex items-start justify-between">
              <div className="flex-1 min-w-0 mr-4">
                <div className="flex items-center gap-2">
                  {!email.is_read && (
                    <span className="w-2 h-2 bg-blue-600 rounded-full flex-shrink-0" />
                  )}
                  <p className="text-sm font-medium text-gray-900 truncate">
                    {email.subject || "(No subject)"}
                  </p>
                </div>
                <p className="text-sm text-gray-500 truncate mt-0.5">
                  {email.from_address}
                </p>
                <p className="text-xs text-gray-400 mt-0.5">
                  {new Date(email.created_at * 1000).toLocaleString()}
                </p>
              </div>
              <button
                onClick={(e) => {
                  e.stopPropagation();
                  onDelete(email.id);
                }}
                className="text-gray-400 hover:text-red-600 transition-colors flex-shrink-0"
                title="Delete"
              >
                <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16" />
                </svg>
              </button>
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}
