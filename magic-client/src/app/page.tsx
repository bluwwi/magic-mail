"use client";

import Home from "@/components/Home";

const STORAGE_KEY = "temp_mail_address";

export default function Page() {
  return (
    <main className="min-h-screen w-full h-full flex flex-col items-center justify-center bg-amber-600">
      <div className="w-full min-h-screen font-semibold p-4 border-4 bg-amber-100 border-black ">
        <div className="w-full text-5xl arr lowercase text-black">
          MAGIC MAIL
        </div>

        <div className=""></div>
        {/*<div className="w-full text-4xl yuyu text-black">
          free, easy to use and reliable temporary email address
        </div>*/}
      </div>
    </main>
  );
}
