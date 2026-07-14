"use client";

import { useState, useEffect, useCallback } from "react";
import Image from "next/image";
import * as api from "@/lib/api";
import { useInbox } from "@/hooks/useInbox";
import InboxList from "@/components/InboxList";
import EmailViewer from "@/components/EmailViewer";

const STORAGE_KEY = "temp_mail_address";

export default function Page() {
  const [currentAddress, setCurrentAddress] = useState<api.Address | null>(null);
  const [pageLoading, setPageLoading] = useState(true);
  const [IsCopied, setIsCopied] = useState(false);

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

  useEffect(() => {
    try {
      const saved = localStorage.getItem(STORAGE_KEY);
      if (saved) {
        const parsed = JSON.parse(saved) as api.Address;
        if (parsed.expires_at > Math.floor(Date.now() / 1000)) {
          setCurrentAddress(parsed);
          setPageLoading(false);
          return;
        }
      }
    } catch {
      // ignore
    }

    api.generateAddress()
      .then((addr) => {
        setCurrentAddress(addr);
        localStorage.setItem(STORAGE_KEY, JSON.stringify(addr));
      })
      .catch(console.error)
      .finally(() => setPageLoading(false));
  }, []);

  const handleCopy = useCallback(async () => {
    if (!currentAddress) return;
    try {
      await navigator.clipboard.writeText(currentAddress.address);
    } catch {
      const input = document.createElement("input");
      input.value = currentAddress.address;
      document.body.appendChild(input);
      input.select();
      document.execCommand("copy");
      document.body.removeChild(input);
    }
    setIsCopied(true);
    setTimeout(() => setIsCopied(false), 2000);
  }, [currentAddress]);

  const handleRegenerate = useCallback(async () => {
    try {
      const addr = await api.generateAddress();
      setCurrentAddress(addr);
      localStorage.setItem(STORAGE_KEY, JSON.stringify(addr));
      setIsCopied(false);
    } catch (err) {
      console.error("Failed to generate address:", err);
    }
  }, []);

  if (pageLoading) {
    return (
      <main className="min-h-screen w-full bg-[#8D75E6] flex items-center justify-center">
        <div className="text-white text-lg">Loading...</div>
      </main>
    );
  }

  return (
    <main className="min-h-screen w-full h-full flex flex-col items-center bg-[#8D75E6]">
      <div className="w-full items-center flex flex-col min-h-screen justify-start">
        <div className="w-full items-center flex flex-col gap-2">
          <div className="bg-white w-full text-center text-sm py-2 text-black">
            Forget about spam, advertising mailings and hacking keep your
            mailbox clean. Magic Mail provides temporary,secure, anonymous,
            free, disposable email address.
          </div>
          <div className="w-full py-10 mt-4">
            <div className="w-full text-8xl rek mb-2 text-center text-white">
              Magic Mail
            </div>
            <div className="arr text-xl text-center text-white leading-tight">
              Receive emails anonymously with our free, private, and secure{" "}
              <br /> temporary email address generator.
            </div>
          </div>
          <div className="flex gap-2 items-center">
            <div className="rounded-full items-center pl-4 pr-1.5 py-1.5 flex gap-2 bg-black/90 w-fit">
              <div className="text-base text-white select-all">
                {currentAddress?.address || "Generating..."}
              </div>
              <div
                className="bg-white/90 select-none rounded-full p-2.5 cursor-pointer"
                onClick={handleCopy}
              >
                {IsCopied ? (
                  <Image
                    src="/icons/copy-suc.svg"
                    alt="copied"
                    width={24}
                    height={24}
                    className="w-6 invert"
                  />
                ) : (
                  <Image
                    src="/icons/copy.svg"
                    alt="copy"
                    width={24}
                    height={24}
                    className="w-6 invert"
                  />
                )}
              </div>
            </div>
            <div
              className="rounded-full cursor-pointer select-none items-center pl-4 pr-1.5 py-1.5 flex gap-2 bg-black/90 w-fit"
              onClick={handleRegenerate}
            >
              <div className="text-base text-white">regenerate</div>
              <Image
                src="/icons/refresh.svg"
                alt="refresh"
                width={40}
                height={40}
                className="w-10 opacity-85"
              />
            </div>
          </div>

          {error && (
            <div className="mt-4 px-4 py-2 bg-red-500/20 border border-red-500/40 rounded-lg text-sm text-white">
              {error}
            </div>
          )}

          {currentAddress && (
            <div className="bg-black w-[85%] h-160 p-5 rounded-4xl my-10 flex gap-0">
              <div className="w-[30%] h-full flex flex-col">
                <InboxList
                  emails={emails}
                  selectedId={selectedEmail?.id ?? null}
                  loading={loading}
                  onSelect={selectEmail}
                  onDelete={deleteEmail}
                  onClear={clearEmails}
                />
              </div>
              <div className="w-[70%] h-full border-l border-white/20 pl-5">
                {selectedEmail ? (
                  <EmailViewer
                    email={selectedEmail}
                    onClose={deselectEmail}
                    onDelete={deleteEmail}
                  />
                ) : (
                  <div className="flex items-center justify-center h-full text-white/40 text-sm">
                    Select an email to view
                  </div>
                )}
              </div>
            </div>
          )}
        </div>
      </div>
    </main>
  );
}
