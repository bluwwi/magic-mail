"use client";

import { useState, useEffect, useCallback } from "react";
import AddressBar from "../components/AddressBar";
import InboxList from "../components/InboxList";
import EmailViewer from "../components/EmailViewer";
import { useInbox } from "../hooks/useInbox";
import * as api from "../lib/api";

const STORAGE_KEY = "temp_mail_address";

export default function Home() {
  const [currentAddress, setCurrentAddress] = useState<api.Address | null>(null);
  const [savedAddress, setSavedAddress] = useState<api.Address | null>(null);
  const [pageLoading, setPageLoading] = useState(true);

  useEffect(() => {
    try {
      const saved = localStorage.getItem(STORAGE_KEY);
      if (saved) {
        const parsed = JSON.parse(saved) as api.Address;
        if (parsed.expires_at > Math.floor(Date.now() / 1000)) {
          setSavedAddress(parsed);
          setCurrentAddress(parsed);
        }
      }
    } catch {
      // Ignore
    }
    setPageLoading(false);
  }, []);

  const handleAddressChange = useCallback((addr: api.Address) => {
    localStorage.setItem(STORAGE_KEY, JSON.stringify(addr));
    setCurrentAddress(addr);
  }, []);

  const {
    emails,
    selectedEmail,
    loading,
    error,
    selectEmail,
    deselectEmail,
    deleteEmail,
    clearEmails,
  } = useInbox(currentAddress);

  if (pageLoading) {
    return (
      <main className="min-h-screen bg-[var(--bg-primary)] flex items-center justify-center">
        <div className="flex flex-col items-center gap-3">
          <div className="w-8 h-8 border-2 border-[var(--accent)] border-t-transparent rounded-full animate-spin" />
          <p className="text-[var(--text-secondary)] text-sm">Loading...</p>
        </div>
      </main>
    );
  }

  return (
    <main className="min-h-screen bg-[var(--bg-primary)]">
      <div className="max-w-4xl mx-auto px-4 py-8 sm:py-12">
        <div className="text-center mb-10">
          <h1 className="text-4xl sm:text-5xl font-bold tracking-tight text-[var(--text-primary)]">
            Temp<span className="text-[var(--accent)]">Mail</span>
          </h1>
          <p className="mt-3 text-[var(--text-secondary)] text-sm sm:text-base">
            Temporary email inbox — receive emails without revealing your real address.
          </p>
        </div>

        <div className="animate-fade-in">
          <AddressBar
            initialAddress={savedAddress}
            onAddressChange={handleAddressChange}
          />
        </div>

        {error && (
          <div className="mt-4 p-3 bg-[var(--danger-bg)] border border-[var(--danger)]/30 rounded-lg text-sm text-[var(--danger-hover)] animate-slide-in">
            {error}
          </div>
        )}

        {currentAddress && currentAddress.expires_at < Math.floor(Date.now() / 1000) && (
          <div className="mt-4 p-3 bg-[var(--warning-bg)] border border-[var(--warning)]/30 rounded-lg text-sm text-[var(--warning)] animate-slide-in">
            This address has expired. New emails will not be delivered. Generate a new address to continue.
          </div>
        )}

        {currentAddress && (
          <div className="mt-8 animate-fade-in">
            <div className="lg:grid lg:grid-cols-2 lg:gap-6">
              <div className={`${selectedEmail ? "hidden lg:block" : ""}`}>
                <div className="flex items-center justify-between mb-3">
                  <h2 className="text-lg font-semibold text-[var(--text-primary)]">Inbox</h2>
                  {emails.length > 0 && (
                    <span className="text-xs text-[var(--text-muted)]">
                      {emails.length} {emails.length === 1 ? "message" : "messages"}
                    </span>
                  )}
                </div>
                <InboxList
                  emails={emails}
                  selectedId={selectedEmail?.id ?? null}
                  loading={loading}
                  onSelect={selectEmail}
                  onDelete={deleteEmail}
                  onClear={clearEmails}
                />
              </div>

              <div className={`${!selectedEmail ? "hidden lg:block" : ""}`}>
                <h2 className="text-lg font-semibold text-[var(--text-primary)] mb-3">Message</h2>
                {selectedEmail ? (
                  <EmailViewer
                    email={selectedEmail}
                    onClose={deselectEmail}
                    onDelete={deleteEmail}
                  />
                ) : (
                  <div className="bg-[var(--bg-card)] rounded-xl border border-[var(--border)] p-12 text-center">
                    <div className="text-4xl mb-4 opacity-30">💬</div>
                    <p className="text-[var(--text-muted)] text-sm">
                      Select an email to view its contents
                    </p>
                  </div>
                )}
              </div>
            </div>
          </div>
        )}

        {!currentAddress && !pageLoading && (
          <div className="mt-12 text-center animate-fade-in">
            <p className="text-[var(--text-muted)] text-sm">
              Generating your temporary address...
            </p>
          </div>
        )}
      </div>
    </main>
  );
}
