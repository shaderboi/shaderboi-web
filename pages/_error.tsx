import React, { useEffect } from "react";
import Router from "next/router";

const Error = ({ statusCode }: any) => {
  return (
    <div>
      <p>{`Error ${statusCode}`}</p>
    </div>
  );
};

Error.getInitialProps = ({ res, err }: any) => {
  const statusCode = res ? res.statusCode : err ? err.statusCode : 404;
  return { statusCode };
};

export default Error;
