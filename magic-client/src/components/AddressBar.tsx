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
  const autoGenDone = useCallback(() => {}, []);

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
    <div className="bg-white rounded-lg shadow-sm border border-gray-200 p-4">
      <div className="flex flex-col sm:flex-row gap-3 items-start sm:items-center">
        {domains.length > 1 ? (
          <select
            value={selectedDomain}
            onChange={(e) => setSelectedDomain(e.target.value)}
            className="px-3 py-2 border border-gray-300 rounded-md text-sm focus:outline-none focus:ring-2 focus:ring-blue-500"
          >
            <option value="">Random domain</option>
            {domains.map((d) => (
              <option key={d} value={d}>
                @{d}
              </option>
            ))}
          </select>
        ) : domains.length === 1 ? (
          <span className="text-sm text-gray-500">@{domains[0]}</span>
        ) : null}

        <button
          onClick={handleGenerate}
          disabled={loading}
          className="px-4 py-2 bg-blue-600 text-white rounded-md text-sm font-medium hover:bg-blue-700 disabled:opacity-50 transition-colors"
        >
          {loading ? "Generating..." : "Generate New Address"}
        </button>

        {address && (
          <div className="flex items-center gap-3 ml-auto">
            <span className="text-sm font-mono text-gray-700">{address.address}</span>
            <button
              onClick={handleCopy}
              className="px-3 py-1.5 text-xs border border-gray-300 rounded-md hover:bg-gray-50 transition-colors"
            >
              {copied ? "Copied!" : "Copy"}
            </button>
            {timeLeft !== null && (
              <span
                className={`text-xs font-mono ${
                  timeLeft < 60 ? "text-red-500" : "text-gray-500"
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
