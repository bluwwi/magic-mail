"use client";

import { useState, useEffect, useCallback } from "react";
import * as api from "@/lib/api";

interface AddressBarProps {
  initialAddress?: api.Address | null;
  onAddressChange: (address: api.Address) => void;
}

export default function AddressBar({ initialAddress = null, onAddressChange }: AddressBarProps) {
  const [address, setAddress] = useState<api.Address | null>(initialAddress);
  const [domains, setDomains] = useState<string[]>([]);
  const [selectedDomain, setSelectedDomain] = useState<string>("");
  const [loading, setLoading] = useState(false);
  const [copied, setCopied] = useState(false);
  const [timeLeft, setTimeLeft] = useState<number | null>(null);

  useEffect(() => {
    api.getDomains().then(setDomains).catch(console.error);
  }, []);

  useEffect(() => {
    if (initialAddress) {
      setAddress(initialAddress);
    }
  }, [initialAddress]);

  useEffect(() => {
    if (address) return;
    let cancelled = false;
    setLoading(true);
    api.generateAddress(selectedDomain || undefined).then((addr) => {
      if (cancelled) return;
      setAddress(addr);
      setCopied(false);
      onAddressChange(addr);
    }).catch(console.error).finally(() => {
      if (!cancelled) setLoading(false);
    });
    return () => { cancelled = true; };
  }, []); // eslint-disable-line react-hooks/exhaustive-deps

  useEffect(() => {
    if (!address) return;
    const update = () => {
      const remaining = Math.max(0, address.expires_at - Math.floor(Date.now() / 1000));
      setTimeLeft(remaining);
    };
    update();
    const id = setInterval(update, 1000);
    return () => clearInterval(id);
  }, [address]);

  const handleGenerate = useCallback(async () => {
    setLoading(true);
    try {
      const addr = await api.generateAddress(selectedDomain || undefined);
      setAddress(addr);
      setCopied(false);
      onAddressChange(addr);
    } catch (err) {
      console.error("Failed to generate address:", err);
    } finally {
      setLoading(false);
    }
  }, [selectedDomain, onAddressChange]);

  const handleCopy = useCallback(async () => {
    if (!address) return;
    try {
      await navigator.clipboard.writeText(address.address);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    } catch {
      const input = document.createElement("input");
      input.value = address.address;
      document.body.appendChild(input);
      input.select();
      document.execCommand("copy");
      document.body.removeChild(input);
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
  }, [address]);

  const formatTime = (seconds: number): string => {
    const m = Math.floor(seconds / 60);
    const s = seconds % 60;
    return `${m}:${s.toString().padStart(2, "0")}`;
  };

  return (
    <div className="bg-[var(--bg-card)] rounded-xl border border-[var(--border)] p-5">
      <div className="flex flex-col sm:flex-row gap-3 items-start sm:items-center">
        <div className="flex items-center gap-3 w-full sm:w-auto">
          {domains.length > 1 && (
            <select
              value={selectedDomain}
              onChange={(e) => setSelectedDomain(e.target.value)}
              className="bg-[var(--bg-input)] text-[var(--text-primary)] border border-[var(--border)] rounded-lg px-3 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-[var(--accent)] focus:border-transparent appearance-none cursor-pointer"
            >
              <option value="">Random domain</option>
              {domains.map((d) => (
                <option key={d} value={d}>
                  @{d}
                </option>
              ))}
            </select>
          )}
          {domains.length === 1 && (
            <span className="text-sm text-[var(--text-secondary)]">@{domains[0]}</span>
          )}
        </div>

        <button
          onClick={handleGenerate}
          disabled={loading}
          className="px-4 py-2.5 bg-[var(--accent)] text-white rounded-lg text-sm font-medium hover:bg-[var(--accent-hover)] disabled:opacity-50 disabled:cursor-not-allowed transition-all duration-200 active:scale-[0.97] whitespace-nowrap"
        >
          {loading ? (
            <span className="flex items-center gap-2">
              <span className="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin" />
              Generating...
            </span>
          ) : (
            "Generate New"
          )}
        </button>

        {address && (
          <div className="flex items-center gap-3 sm:ml-auto w-full sm:w-auto">
            <div className="flex-1 sm:flex-none bg-[var(--bg-input)] border border-[var(--border)] rounded-lg px-3 py-2 flex items-center gap-2 min-w-0">
              <span className="text-sm font-mono text-[var(--text-primary)] truncate">
                {address.address}
              </span>
              <button
                onClick={handleCopy}
                className={`flex-shrink-0 px-2 py-0.5 text-xs font-medium rounded-md transition-all duration-200 ${
                  copied
                    ? "bg-[var(--success-bg)] text-[var(--success)]"
                    : "bg-[var(--accent-glow)] text-[var(--accent)] hover:bg-[var(--accent)] hover:text-white"
                }`}
              >
                {copied ? "Copied!" : "Copy"}
              </button>
            </div>
            {timeLeft !== null && (
              <span
                className={`text-xs font-mono flex-shrink-0 ${
                  timeLeft === 0
                    ? "text-[var(--danger)]"
                    : timeLeft < 60
                    ? "text-[var(--warning)]"
                    : "text-[var(--text-muted)]"
                }`}
              >
                {timeLeft === 0 ? "EXPIRED" : formatTime(timeLeft)}
              </span>
            )}
          </div>
        )}
      </div>
    </div>
  );
}
