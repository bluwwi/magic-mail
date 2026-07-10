"use client";

import * as api from "@/lib/api";

interface EmailViewerProps {
  email: api.Email;
  onClose: () => void;
  onDelete: (id: string) => void;
}

export default function EmailViewer({ email, onClose, onDelete }: EmailViewerProps) {
  return (
    <div className="bg-white rounded-lg shadow-sm border border-gray-200">
      <div className="flex items-center justify-between px-4 py-3 border-b border-gray-200">
        <div className="flex-1 min-w-0">
          <h2 className="text-lg font-semibold text-gray-900 truncate">
            {email.subject || "(No subject)"}
          </h2>
          <p className="text-sm text-gray-500 mt-0.5">
            From: {email.from_address}
          </p>
          <p className="text-sm text-gray-500">
            To: {email.to_address}
          </p>
          <p className="text-xs text-gray-400 mt-0.5">
            {new Date(email.created_at * 1000).toLocaleString()}
          </p>
        </div>
        <div className="flex items-center gap-2 ml-4">
          <button
            onClick={() => onDelete(email.id)}
            className="px-3 py-1.5 text-sm text-red-600 border border-red-300 rounded-md hover:bg-red-50 transition-colors"
          >
            Delete
          </button>
          <button
            onClick={onClose}
            className="px-3 py-1.5 text-sm text-gray-600 border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
          >
            Close
          </button>
        </div>
      </div>

      <div className="px-4 py-4">
        {email.body_html ? (
          <div
            className="prose prose-sm max-w-none"
            dangerouslySetInnerHTML={{ __html: email.body_html }}
          />
        ) : (
          <pre className="text-sm text-gray-700 whitespace-pre-wrap font-sans">
            {email.body_text || "(No content)"}
          </pre>
        )}
      </div>
    </div>
  );
}
