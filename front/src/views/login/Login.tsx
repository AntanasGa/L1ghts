import React, { useContext, useEffect, useMemo, useRef } from "react";
import { FormEvent, useState } from "react";
import { useNavigate } from "react-router-dom";
import { AxiosError } from "axios";
import { useAppDispatch, useAppSelector } from "hooks";
import { setAuthed } from "store/auth";
import { ApiContext } from "utils/api";
import { setLogin } from "utils/cookies";
import Loader, { Stage } from "components/Loader/Loader";

enum LoginState {
  Initializing = 0,
  Ready = 1,
  Loading = 2,
  Complete = 3,
}

export default function Login() {
  const navigate = useNavigate();
  const api = useContext(ApiContext);

  const loggedIn = useAppSelector(state => state.auth.authed);

  const didMount = useRef(false);
  const options = useMemo(() => ["bottom-0 translate-y-100", "bottom-1/2", "bottom-14", "bottom-[-100%]"], []);
  
  const [errs, setErrs] = useState({ user_name: "", password: "" });
  const [loading, setLoading] = useState(false);
  const [loginState, setLoginState] = useState((loggedIn && LoginState.Complete) || LoginState.Initializing);
  const [loaderStage, setLoaderStage] = useState<string | undefined>();
  const cardClassList = useMemo(() =>["absolute flex flex-col items-center card self-center ease-in duration-150 translate-y-1/2", options[loginState]], [loginState, options]);

  
  const dispatch = useAppDispatch();

  useEffect(() => {
    if (loggedIn) {
      navigate("/dashboard");
    }
  }, [navigate, loggedIn]);

  useEffect(() => {
    if (!didMount.current) {
      didMount.current = true;
      setLoginState(LoginState.Ready);
    }
    return () => {
      didMount.current = false;
    };
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [navigate]);

  function handleOnSubmit(e: FormEvent<HTMLFormElement>) {
    e.preventDefault();
    if (!api) {
      return;
    }

    setLoginState(LoginState.Loading);
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
      setLoginState(LoginState.Ready);
      return;
    }

    if (creds.user_name instanceof File || creds.password instanceof File) {
      setErrs({
        user_name: creds.user_name instanceof File ? "Username input was messed with" : "",
        password: creds.password instanceof File ? "Password input was messed with" : "",
      });
      setLoading(false);
      setLoginState(LoginState.Ready);
      return;
    }
    let shouldReset = true;

    api.auth.login({ user_name: creds.user_name, password: creds.password }).then((res) => {
      setLoaderStage("shadow-green-400");
      setTimeout(() => {
        setLoginState(LoginState.Complete);
        shouldReset = false;
        setTimeout(() => {
          setLogin({ ref: res.data.refresh_token, tok: res.data.access_token });
          dispatch(setAuthed(true));
        }, 300);
      }, 100);
    }).catch((err: AxiosError) => {
      console.log(err.response?.data);
      setLoaderStage("shadow-red-400");
    }).finally(() => {
      if (shouldReset) {
        setLoading(false);
        setTimeout(() => {
          setLoginState(LoginState.Ready);
          setLoaderStage(undefined);
        }, 1000);
      }
    });
  }

  return (
    <div className="fixed inset-0 flex items-center place-content-center">
      <div className={ cardClassList.join(" ") }>
        <div className={ ((LoginState.Loading === loginState || LoginState.Complete === loginState) && "hidden") || ""}>
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
        { (LoginState.Loading === loginState || LoginState.Complete === loginState) &&
          <Loader stage={ Stage.active } size={ 16 } color={ loaderStage } />
        }
      </div>
    </div>
  );
}
