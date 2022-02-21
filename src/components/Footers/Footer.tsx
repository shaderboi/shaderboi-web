import React from "react";
import Link from "next/link";
import {
  FaFacebook,
  FaFacebookSquare,
  FaGithub,
  FaInstagram,
  FaMapMarkerAlt,
  FaTwitter,
  FaTwitterSquare,
} from "react-icons/fa";

export default function Footer() {
  return (
    <>
      <footer className="relative bg-blueGray-200 pt-8 pb-6">
        <div
          className="bottom-auto top-0 left-0 right-0 w-full absolute pointer-events-none overflow-hidden -mt-20 h-20"
          style={{ transform: "translateZ(0)" }}
        >
          <svg
            className="absolute bottom-0 overflow-hidden"
            xmlns="http://www.w3.org/2000/svg"
            preserveAspectRatio="none"
            version="1.1"
            viewBox="0 0 2560 100"
            x="0"
            y="0"
          >
            <polygon
              className="text-blueGray-200 fill-current"
              points="2560 0 2560 100 0 100"
            ></polygon>
          </svg>
        </div>
        <div className="container mx-auto px-4" id="footer-content">
          <div className="flex flex-wrap text-center lg:text-left">
            <div className="w-full lg:w-4/12 px-4">
              <h4 className="text-3xl font-semibold">
                {"Let's keep in touch!"}
              </h4>
              <h5 className="text-lg mt-0 mb-2 text-blueGray-600">
                Find us on any of these platforms, we respond 1-2 business days.
              </h5>
              <ul className="list-none text-md">
                <li>Phone: +62 812-9639-0327 (Michael)</li>
                <li>LINE messenger: mike0883 (Michael)</li>
              </ul>
              <div className="mt-6 lg:mb-0 mb-6 flex flex-row gap-2">
                <Link href="https://www.twitter.com/ShaderboiTech">
                  <a>
                    <div className="bg-white rounded-full p-3 w-fit shadow-2xl">
                      <FaTwitter className="text-lightBlue-700" size={20} />
                    </div>
                  </a>
                </Link>
                <Link href="https://www.facebook.com/shaderboi">
                  <a>
                    <div className="bg-white rounded-full p-3 w-fit shadow-2xl">
                      <FaFacebook className="text-lightBlue-500" size={20} />
                    </div>
                  </a>
                </Link>
                <Link href="https://instagram.com/shaderboi">
                  <a>
                    <div className="bg-white rounded-full p-3 w-fit shadow-2xl">
                      <FaInstagram className="text-red-500" size={20} />
                    </div>
                  </a>
                </Link>
                <Link href="https://github.com/shaderboi">
                  <a>
                    <div className="bg-white rounded-full p-3 w-fit shadow-2xl">
                      <FaGithub className="" size={20} />
                    </div>
                  </a>
                </Link>
              </div>
            </div>
            <div className="w-full lg:w-4/12 px-4">
              <div className="flex flex-wrap items-top mb-6 text-center items-center h-full">
                <div className="w-full px-4 ml-auto">
                  <h3 className="block uppercase text-blueGray-500 text-sm font-semibold mb-2">
                    PT. Teknologi Cahaya Grafika
                  </h3>
                  <div className="my-2">
                    <p>JL. Tapir I No. 10 M2 Cikarang</p>
                    <p>17530 Jawa Barat, Indonesia</p>
                  </div>
                  <a
                    href="https://goo.gl/maps/59K7pGtTQa1CK5LW6"
                    className="text-violet-800 flex flex-row justify-center items-center"
                  >
                    <FaMapMarkerAlt size={20} className="mr-2" />
                    Show in Google Maps
                  </a>
                </div>
              </div>
            </div>
            <div className="w-full lg:w-4/12 px-4">
              <div className="flex flex-wrap items-top mb-6">
                <div className="w-full px-4 ml-auto">
                  <span className="block uppercase text-blueGray-500 text-sm font-semibold mb-2">
                    Other Links
                  </span>
                  <ul className="list-unstyled">
                    <li>
                      <a
                        className="text-blueGray-600 hover:text-blueGray-800 font-semibold block pb-2 text-sm"
                        href="https://github.com/shaderboi/shaderboi-web"
                      >
                        Source Code
                      </a>
                    </li>
                    <li>
                      <a
                        className="text-blueGray-600 hover:text-blueGray-800 font-semibold block pb-2 text-sm"
                        href="#"
                      >
                        Blog
                      </a>
                    </li>
                    <li>
                      <Link href="/data-privacy/privacy-policy">
                        <a className="text-blueGray-600 hover:text-blueGray-800 font-semibold block pb-2 text-sm">
                          Privacy Policy
                        </a>
                      </Link>
                    </li>
                    <li>
                      <a
                        className="text-blueGray-600 hover:text-blueGray-800 font-semibold block pb-2 text-sm"
                        href="#"
                      >
                        Terms
                      </a>
                    </li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
          <hr className="my-6 border-blueGray-300" />
          <div className="flex flex-wrap items-center md:justify-between justify-center">
            <div className="w-full md:w-4/12 px-4 mx-auto text-center">
              <div className="text-sm text-blueGray-500 font-semibold py-1">
                Copyright © {new Date().getFullYear()} Shaderboi. Indonesia.
              </div>
            </div>
          </div>
        </div>
      </footer>
    </>
  );
}
