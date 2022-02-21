import React, { useEffect } from "react";
import Router from "next/router";

const Error = ({ statusCode }: any) => {
  return (
    <div className="flex flex-row justify-center items-center h-screen">
      <h2 className="text-4xl font-bold">{`Error ${statusCode}`}</h2>
    </div>
  );
};

export default Error;

Error.getInitialProps = ({ res, err }: any) => {
  const statusCode = res ? res.statusCode : err ? err.statusCode : 404;
  return { statusCode };
};
