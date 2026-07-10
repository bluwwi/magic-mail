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
      <main className="min-h-screen bg-gray-50 flex items-center justify-center">
        <p className="text-gray-500">Loading...</p>
      </main>
    );
  }

  return (
    <main className="min-h-screen bg-gray-50">
      <div className="max-w-4xl mx-auto px-4 py-8">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">TempMail</h1>
        <p className="text-gray-600 mb-8">
          Temporary email inbox — receive emails without revealing your real address.
        </p>

        <AddressBar
          initialAddress={savedAddress}
          onAddressChange={handleAddressChange}
        />

        {error && (
          <div className="mt-4 p-3 bg-red-50 border border-red-200 rounded-md text-sm text-red-700">
            {error}
          </div>
        )}

        {currentAddress && currentAddress.expires_at < Math.floor(Date.now() / 1000) && (
          <div className="mt-4 p-3 bg-yellow-50 border border-yellow-200 rounded-md text-sm text-yellow-800">
            This address has expired. New emails will not be delivered. Generate a new address to continue.
          </div>
        )}

        {currentAddress && (
          <div className="mt-8">
            <div className="lg:grid lg:grid-cols-2 lg:gap-6">
              <div className={`${selectedEmail ? "hidden lg:block" : ""}`}>
                <h2 className="text-lg font-semibold text-gray-900 mb-3">Inbox</h2>
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
                <h2 className="text-lg font-semibold text-gray-900 mb-3">Message</h2>
                {selectedEmail ? (
                  <EmailViewer
                    email={selectedEmail}
                    onClose={deselectEmail}
                    onDelete={deleteEmail}
                  />
                ) : (
                  <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-8 text-center text-gray-400">
                    Select an email to view its contents
                  </div>
                )}
              </div>
            </div>
          </div>
        )}
      </div>
    </main>
  );
}
