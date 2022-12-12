import { useAppSelector } from "hooks";
import React from "react";
import { Link } from "react-router-dom";

export default function CatchAll() {
  const authed = useAppSelector(store => store.auth.authed);

  const location = authed ? "/dashboard" : "/";
  return (
    <div className="flex items-center place-content-center h-screen">
      <div className="flex flex-col items-center card self-center">
        <div className="text-6xl font-black flex items-baseline">
          <div className="-rotate-45">?</div>
          <h1 className="text-9xl">?</h1>
          <div className="rotate-45">?</div>
        </div>
        <h1 className="text-3xl font-black">Seems like you got lost in the darkness</h1>
        <Link to={ location } className="text-emerald-500 dark:text-emerald-400">Go back to the light</Link>
      </div>
    </div>
  );
}
