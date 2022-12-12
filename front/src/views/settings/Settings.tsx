import { AxiosError } from "axios";
import { useAppDispatch } from "hooks";
import React, { useContext, MouseEvent } from "react";
import { setAuthed } from "store/auth";
import { ApiContext } from "utils/api";
import { resetLogin } from "utils/cookies";

export default function Settings() {
  const api = useContext(ApiContext);
  const dispatch = useAppDispatch();
  function logout(_: MouseEvent<HTMLButtonElement>) {
    api?.auth.logout()
      .catch((e: AxiosError) => {
        console.log(e.message);
      })
      .finally(() => {
        resetLogin();
        dispatch(setAuthed(false));
      });
  }
  return (
    <div className="flex flex-col">
      <h1 className="font-bold text-4xl">Account</h1>
      <button className="btn primary" type="button" onClick={ logout }>Logout</button>
    </div>
  );
}
