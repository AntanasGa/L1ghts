import React, { useContext, useEffect } from "react";
import { FormEvent, useState } from "react";
import { useNavigate } from "react-router-dom";
import { AxiosError } from "axios";
import { useAppDispatch, useAppSelector } from "hooks";
import { setAuthed } from "store/auth";
import { ApiContext } from "utils/api";
import { setLogin } from "utils/cookies";

export default function Login() {
  const navigate = useNavigate();
  const api = useContext(ApiContext);
  const [errs, setErrs] = useState({ user_name: "", password: "" });
  const [loading, setLoading] = useState(false);
  const loggedIn = useAppSelector(state => state.auth.authed);
  const dispatch = useAppDispatch();

  useEffect(() => {
    if (loggedIn) {
      navigate("/dashboard");
    }
  }, [navigate, loggedIn]);

  function handleOnSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    if (!api) {
      return;
    }
    setLoading(true);
    const data = new FormData(e.currentTarget);
    if (!data.has("user_name") || !data.has("password")) {
      setErrs({
        user_name: data.has("user_name") ? "" : "Username missing",
        password: data.has("password") ? "" : "Password missing",
      });
      setLoading(false);
      return;
    }
    const creds = {
      user_name: data.get("user_name"),
      password: data.get("password"),
    };
    if (!creds.user_name || !creds.password) {
      setErrs({
        user_name: creds.user_name ? "" : "Username empty",
        password: creds.password ? "" : "Password empty",
      });
      setLoading(false);
      return;
    }
    if (creds.user_name instanceof File || creds.password instanceof File) {
      setErrs({
        user_name: creds.user_name instanceof File ? "Username input was messed with" : "",
        password: creds.password instanceof File ? "Password input was messed with" : "",
      });
      setLoading(false);
      return;
    }
    api.auth.login({ user_name: creds.user_name, password: creds.password }).then((res) => {
      setLogin({ ref: res.data.refresh_token, tok: res.data.access_token });
      dispatch(setAuthed(true));
    }).catch((err: AxiosError) => {
      console.log(err.response?.data);
    }).finally(() => {
      setLoading(false);
    });
  }

  return (
    <div className="flex items-center place-content-center h-screen">
      <div className="flex flex-col items-center card self-center">
        <h1 className="text-6xl">login</h1>
        <form className="flex flex-col pt-8" onSubmit={handleOnSubmit}>
          {errs.user_name && <p className="text-red-600">{errs.user_name}</p>}
          {errs.password && <p className="text-red-600">{errs.password}</p>}
          <label htmlFor="user_name">Username:</label>
          <input
            type="text"
            name="user_name"
            id="user_name"
            autoComplete="username"
            disabled={loading}
            onInput={() => { setErrs((e) => ({ user_name: "", password: e.password })); }}
          />
          <label htmlFor="password">Password:</label>
          <input
            type="password"
            name="password"
            id="password"
            disabled={loading}
            onInput={() => { setErrs((e) => ({ user_name: e.user_name, password: "" })); }}
          />
          <input
            type="submit"
            value="Login"
            className="btn primary mt-2"
            disabled={loading}
          />
        </form>
      </div>
    </div>
  );
}
