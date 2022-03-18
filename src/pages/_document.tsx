import React from "react";
import Document, { Html, Head, Main, NextScript, DocumentContext, DocumentInitialProps } from "next/document";

class CustomDocument extends Document {
  static async getInitialProps(ctx: DocumentContext) {
      const initialProps = await Document.getInitialProps(ctx)
      return { ...initialProps }
  }

  render() {
    return (
      <Html lang="en">
        <Head>
          <meta charSet="utf-8" />
          <meta name="theme-color" content="#000000" />
          <link rel="apple-touch-icon" sizes="180x180" href="/img/icon/apple-touch-icon.png" />
          <link rel="icon" type="image/png" sizes="32x32" href="/img/icon/favicon-32x32.png" />
          <link rel="icon" type="image/png" sizes="16x16" href="/img/icon/favicon-16x16.png" />
          <link rel="manifest" href="/img/icon/site.webmanifest" />
          <link rel="mask-icon" href="/img/icon/safari-pinned-tab.svg" color="#5bbad5" />
          <meta name="msapplication-TileColor" content="#da532c" />
          <meta name="theme-color" content="#ffffff" />
        </Head>
        <body className="text-blueGray-700 antialiased">
          <div id="page-transition"></div>
          <Main />
          <NextScript />
        </body>
      </Html>
    );
  }
}

export default CustomDocument;
