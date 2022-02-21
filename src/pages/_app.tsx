import React from "react";
import { AppProps } from "next/app";
import Head from "next/head";

import "../../styles/font/inter/inter.css";
import "../../styles/globals.css";

import IndexNavbar from "../components/Navbars/IndexNavbar";
import Footer from "../components/Footers/Footer";

function App({ Component, pageProps }: AppProps) {
  return (
    <>
      <Head>
        <meta
          name="viewport"
          content="width=device-width, initial-scale=1, shrink-to-fit=no"
        />
        <title>
          Shaderboi | A place where you can find a solution for your problem.
        </title>
      </Head>
      <>
        <IndexNavbar />
        <div className="mt-[76px]" />
        <Component {...pageProps} />
        <div className="pb-40" />
        <Footer />
      </>
    </>
  );
}

export default App;
