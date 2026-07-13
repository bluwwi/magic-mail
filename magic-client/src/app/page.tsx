"use client";

import Home from "@/components/Home";
import Image from "next/image";
import { useState } from "react";

const STORAGE_KEY = "temp_mail_address";

export default function Page() {
  const [IsCopied, setIsCopied] = useState(false);
  const handleCopy = () => {
    setIsCopied(true);
  };
  return (
    <main className="min-h-screen w-full h-full flex flex-col items-center justify-center bg-[#8D75E6]">
      <div className="w-full  items-center flex flex-col min-h-screen justify-start border-4 border-white ">
        <div className="w-full items-center flex flex-col gap-2 ">
          <div className="bg-white w-full text-center text-sm py-2 text-black ">
            Forget about spam, advertising mailings and hacking keep your
            mailbox clean. Magic Mail provides temporary,secure, anonymous,
            free, disposable email address.
          </div>
          <div className="w-full py-10 mt-4">
            <div className="w-full  text-8xl rek mb-2 text-center  text-white">
              Magic Mail
            </div>

            <div className="arr  text-xl text-center  text-white leading-tight">
              Receive emails anonymously with our free, private, and secure{" "}
              <br /> temporary email address generator.
            </div>
          </div>
          <div className="flex gap-2">
            <div className="rounded-full items-center pl-4 pr-1.5 py-1.5 flex gap-2 bg-black/90 w-fit">
              <div className="text-base">mradbc992@realblue.lol</div>
              <div
                className="bg-white/90 select-none  rounded-full p-2.5 cursor-pointer"

                onClick={handleCopy}
              >
                {IsCopied ? (
                  <Image
                    src={"/icons/copy-suc.svg"}
                    alt="copy"
                    width={200}
                    height={200}
                    className="w-6 invert"
                  />
                ) : (
                  <Image
                    src={"/icons/copy.svg"}
                    alt="copy"
                    width={200}
                    height={200}
                    className="w-6 invert"
                  />
                )}
              </div>
            </div>
            <div className="rounded-full  cursor-pointer select-none items-center pl-4 pr-1.5  py-1.5 flex gap-2 bg-black/90 w-fit">
              <div className="text-base">regenerate</div>
              <Image
                src={"/icons/refresh.svg"}
                alt="refresh"
                width={200}
                height={200}
                className="w-10 opacity-85"
              />
            </div>
          </div>
          {/*inbox section*/}
          <div className="bg-black w-[85%] h-160 p-5 rounded-4xl my-10">
            {/*mails*/}

            <div className="w-[30%] border-r border-r-white/60 h-full"></div>
            {/*body*/}
            <div className="w-[70%] h-full"></div>
          </div>
        </div>
      </div>
    </main>
  );
}
