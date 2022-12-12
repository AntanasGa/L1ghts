import { useAppDispatch } from "hooks";
import React, { useEffect } from "react";
import { Outlet } from "react-router-dom";
import { setAuthed } from "store/auth";
import api, { ApiContext } from "utils/api";

export default function Api() {
  // const navigate = useNavigate();
  const dispatch = useAppDispatch();

  // Auth related enough to put auth action here
  useEffect(() => {
    const cookies = Object.fromEntries(document?.cookie.split(";").map(e => {
      const [key, val] = e.trim().split("=", 2);
      return [key, val];
    }));
    dispatch(setAuthed("auth.r" in cookies && "auth.t" in cookies));
  }, [dispatch]);

  const providerValue = api(dispatch);

  return (
    <ApiContext.Provider value={ providerValue }><Outlet /></ApiContext.Provider>
  );
}