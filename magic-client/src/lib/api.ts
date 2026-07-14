const BASE = "";

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

export interface DeleteResponse {
  deleted: boolean;
}

export interface ClearResponse {
  deleted_count: number;
}

export async function getHealth(): Promise<{ status: string; uptime_seconds: number; db_connected: boolean; version: string }> {
  const res = await fetch(`${BASE}/api/health`);
  if (!res.ok) throw new Error(`Health check failed: ${res.status}`);
  return res.json();
}

export async function getDomains(): Promise<string[]> {
  const res = await fetch(`${BASE}/api/domains`);
  if (!res.ok) throw new Error(`Failed to fetch domains: ${res.status}`);
  return res.json();
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

export async function getEmail(address: string, id: string): Promise<Email> {
  const res = await fetch(`${BASE}/api/emails/${encodeURIComponent(address)}/${encodeURIComponent(id)}`);
  if (!res.ok) throw new Error(`Failed to get email: ${res.status}`);
  return res.json();
}

export async function deleteEmail(address: string, id: string): Promise<DeleteResponse> {
  const res = await fetch(`${BASE}/api/emails/${encodeURIComponent(address)}/${encodeURIComponent(id)}`, {
    method: "DELETE",
  });
  if (!res.ok) throw new Error(`Failed to delete email: ${res.status}`);
  return res.json();
}

export async function clearEmails(address: string): Promise<ClearResponse> {
  const res = await fetch(`${BASE}/api/emails/${encodeURIComponent(address)}`, {
    method: "DELETE",
  });
  if (!res.ok) throw new Error(`Failed to clear emails: ${res.status}`);
  return res.json();
}

export function createEmailEventSource(address: string): EventSource {
  return new EventSource(`${BASE}/sse/inbox/${encodeURIComponent(address)}`);
}
