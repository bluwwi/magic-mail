"use client";

import { useState, useEffect, useCallback, useRef } from "react";
import * as api from "@/lib/api";

const POLL_INTERVAL_MS = 5000;
const POLL_MAX_INTERVAL_MS = 30000;

interface UseInboxResult {
  emails: api.Email[];
  selectedEmail: api.Email | null;
  loading: boolean;
  error: string | null;
  selectEmail: (email: api.Email) => void;
  deselectEmail: () => void;
  deleteEmail: (id: string) => Promise<void>;
  clearEmails: () => Promise<void>;
}

export function useInbox(address: api.Address | null): UseInboxResult {
  const [emails, setEmails] = useState<api.Email[]>([]);
  const [selectedEmail, setSelectedEmail] = useState<api.Email | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const eventSourceRef = useRef<EventSource | null>(null);
  const pollingRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const backoffRef = useRef<number>(POLL_INTERVAL_MS);

  const cleanup = useCallback(() => {
    if (eventSourceRef.current) {
      eventSourceRef.current.close();
      eventSourceRef.current = null;
    }
    if (pollingRef.current) {
      clearTimeout(pollingRef.current);
      pollingRef.current = null;
    }
    backoffRef.current = POLL_INTERVAL_MS;
  }, []);

  const fetchEmails = useCallback(async () => {
    if (!address) return;
    setLoading(true);
    setError(null);
    try {
      const list = await api.listEmails(address.address);
      list.sort((a, b) => b.received_at - a.received_at);
      setEmails(list);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to fetch emails");
    } finally {
      setLoading(false);
    }
  }, [address]);

  useEffect(() => {
    if (!address) {
      setEmails([]);
      setSelectedEmail(null);
      cleanup();
      return;
    }

    cleanup();
    fetchEmails();

    let es: EventSource;
    try {
      es = api.createEmailEventSource(address.address);
      eventSourceRef.current = es;

      es.addEventListener("open", () => {
        setError(null);
      });

      es.addEventListener("new_email", () => {
        fetchEmails();
      });

      es.addEventListener("error", () => {
        es.close();
        eventSourceRef.current = null;
        startPolling();
      });
    } catch {
      startPolling();
    }

    function startPolling() {
      const tick = async () => {
        if (!address) return;
        try {
          const list = await api.listEmails(address.address);
          list.sort((a, b) => b.received_at - a.received_at);
          setEmails(list);
          setError(null);
          backoffRef.current = POLL_INTERVAL_MS;
        } catch {
          backoffRef.current = Math.min(
            backoffRef.current * 2,
            POLL_MAX_INTERVAL_MS,
          );
        }
        pollingRef.current = setTimeout(tick, backoffRef.current);
      };
      pollingRef.current = setTimeout(tick, backoffRef.current);
    }

    return cleanup;
  }, [address, fetchEmails, cleanup]);

  const selectEmail = useCallback((email: api.Email) => {
    setEmails((prev) =>
      prev.map((e) => (e.id === email.id ? { ...e, is_read: true } : e)),
    );
    setSelectedEmail(email);
  }, []);

  const deselectEmail = useCallback(() => {
    setSelectedEmail(null);
  }, []);

  const deleteEmail = useCallback(async (id: string) => {
    if (!address) return;
    try {
      await api.deleteEmail(address.address, id);
      setEmails((prev) => prev.filter((e) => e.id !== id));
      setSelectedEmail((prev) => (prev?.id === id ? null : prev));
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to delete email");
    }
  }, [address]);

  const clearEmails = useCallback(async () => {
    if (!address) return;
    try {
      await api.clearEmails(address.address);
      setEmails([]);
      setSelectedEmail(null);
    } catch (err) {
      setError(err instanceof Error ? err.message : "Failed to clear emails");
    }
  }, [address]);

  return {
    emails,
    selectedEmail,
    loading,
    error,
    selectEmail,
    deselectEmail,
    deleteEmail,
    clearEmails,
  };
}
