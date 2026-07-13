"use client";

import Home from "@/components/Home";

const STORAGE_KEY = "temp_mail_address";

export default function Page() {
  return (
    <main className="min-h-screen w-full h-full flex flex-col items-center justify-center bg-[#8D75E6]">
      <div className="w-full  items-center flex flex-col min-h-screen justify-center p-4 border-4 border-white ">
        <div className="w-full  items-center flex flex-col justify-center  ">
          <div className="w-full text-8xl rek text-center  text-white">
            Magic Mail
          </div>

          <div className="arr text-xl text-center  text-white leading-tight">
            Receive emails anonymously with our free, private, and secure <br />{" "}
            temporary email address generator.
          </div>
          {/*<div className="w-full text-4xl yuyu text-black">
          free, easy to use and reliable temporary email address
        </div>*/}
        </div>
      </div>
    </main>
  );
}
