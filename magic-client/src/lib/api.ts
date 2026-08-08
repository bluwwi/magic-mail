// In production on Vercel, set NEXT_PUBLIC_API_URL to your Render backend's HTTPS URL
// (e.g. https://magic-mail.onrender.com). When unset (local dev), requests stay
// same-origin and are proxied to the backend via next.config.ts rewrites.
const BASE = process.env.NEXT_PUBLIC_API_URL ?? "";

export interface Address {
  id: string;
  address: string;
  domain: string;
  created_at: number;
  expires_at: number;
}

export interface Email {
  id: string;
  to_address: string;
  from_addr: string;
  subject: string;
  body_text: string | null;
  body_html: string | null;
  received_at: number;
  is_read: boolean;
}

export async function generateAddress(domain?: string): Promise<Address> {
  const res = await fetch(`${BASE}/api/address/generate`, {
    method: "POST",
    headers: { "Content-Type": "application/json" },
    body: JSON.stringify(domain ? { domain } : {}),
  });
  if (!res.ok) throw new Error(`Failed to generate address: ${res.status}`);
  return res.json();
}

export async function listEmails(address: string): Promise<Email[]> {
  const res = await fetch(`${BASE}/api/emails/${encodeURIComponent(address)}`);
  if (!res.ok) throw new Error(`Failed to list emails: ${res.status}`);
  return res.json();
}

export async function deleteEmail(address: string, id: string): Promise<void> {
  const res = await fetch(
    `${BASE}/api/emails/${encodeURIComponent(address)}/${encodeURIComponent(id)}`,
    { method: "DELETE" },
  );
  if (!res.ok) throw new Error(`Failed to delete email: ${res.status}`);
}

export async function clearEmails(address: string): Promise<void> {
  const res = await fetch(`${BASE}/api/emails/${encodeURIComponent(address)}`, {
    method: "DELETE",
  });
  if (!res.ok) throw new Error(`Failed to clear emails: ${res.status}`);
}

export function createEmailEventSource(address: string): EventSource {
  return new EventSource(`${BASE}/sse/inbox/${encodeURIComponent(address)}`);
}
