import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "TempMail — Temporary Email Service",
  description:
    "Generate temporary email addresses and receive emails in real-time.",
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" className={`manrope h-full antialiased`}>
      <body className="min-h-full flex flex-col">{children}</body>
    </html>
  );
}
