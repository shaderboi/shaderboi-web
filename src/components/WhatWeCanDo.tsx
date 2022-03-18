import { FaGamepad, FaGlobe, FaMobile, FaPalette } from "react-icons/fa";

export const WhatWeCanDo = () => {
  return (
    <>
      <h5 className="text-3xl mt-5 mb-1 font-semibold text-center leading-normal tracking-tight">
        What we provides
      </h5>
      <div className="w-full flex flex-col md:flex-row mt-5">
        <div className="relative flex flex-col w-full">
          <div className="px-4 py-5 flex-auto">
            <div className="bg-white rounded-full p-5 mb-5 w-fit shadow-2xl">
              <FaPalette size={20} />
            </div>
            <h6 className="text-xl mb-1 font-semibold">Design</h6>
            <p className="mb-4 text-blueGray-500">
              Deliver your business with the best visual design and
              interactivity.
            </p>
          </div>
        </div>
        <div className="relative flex flex-col w-full">
          <div className="px-4 py-5 flex-auto">
            <div className="bg-white rounded-full p-5 mb-5 w-fit shadow-2xl">
              <FaMobile size={20} />
            </div>
            <h6 className="text-xl mb-1 font-semibold">
              Mobile Application Development
            </h6>
            <p className="mb-4 text-blueGray-500">
              Make your business accessible quickly in a users hand.
            </p>
          </div>
        </div>
        <div className="relative flex flex-col w-full">
          <div className="px-4 py-5 flex-auto">
            <div className="bg-white rounded-full p-5 mb-5 w-fit shadow-2xl">
              <FaGlobe size={20} />
            </div>
            <h6 className="text-xl mb-1 font-semibold">Website Development</h6>
            <p className="mb-4 text-blueGray-500">
              Leverage your business through internet. Accessible by everyone
              with a browser.
            </p>
          </div>
        </div>
        <div className="relative flex flex-col w-full">
          <div className="px-4 py-5 flex-auto">
            <div className="bg-white rounded-full p-5 mb-5 w-fit shadow-2xl">
              <FaGamepad size={20} />
            </div>
            <h6 className="text-xl mb-1 font-semibold">Game Development</h6>
            <p className="mb-4 text-blueGray-500">
              We are able to create a 2D and 3D games to entertain your
              audience.
            </p>
          </div>
        </div>
      </div>
    </>
  );
};
