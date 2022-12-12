import axios, { AxiosError, AxiosResponse, RawAxiosRequestHeaders } from "axios";
import { useAppDispatch } from "hooks";
import React from "react";
import { setAuthed } from "store/auth";
import provider from "./api/provider";
import { getCookies, resetLogin, setLogin } from "./cookies";

export const ApiContext = React.createContext<ReturnType<typeof provider> | undefined>(undefined);

export default function useApi(dispatch: ReturnType<typeof useAppDispatch>) {
  const host = window?.location.host.split(":", 1)[0] || "localhost";
  const api =  axios.create({
    baseURL: !process.env.NODE_ENV || process.env.NODE_ENV === "development" ? `http://${host}:8080/api` : "/api",
  });
  api.interceptors.request.use(
    (config) => {
      const cookies = getCookies();
      if (!cookies["auth.t"]) {
        console.debug("missing token auth.t in", cookies);
        return config;
      }
      config.headers = config.headers || {};
      config.headers["Authorization"] = `Bearer ${cookies["auth.t"]}`;
      return config;
    },
  );
  
  api.interceptors.response.use(
    function(res) {
      return res;
    },
    async function(err: AxiosError & {_retry?: boolean}) {
      if (!err.response || err.response.status !== 401) {
        return Promise.reject(err);
      }

      if (err._retry) {
        dispatch(setAuthed(false));
        return Promise.reject(err);
      }
      err._retry = true;

      const cookies = getCookies();
      if (!cookies["auth.r"]) {
        resetLogin();
        dispatch(setAuthed(false));
        return Promise.reject(err);
      }

      if (err.config?.url === "/auth/refresh") {
        resetLogin();
        dispatch(setAuthed(false));
        return Promise.reject(err);
      }
      if (!err.config) {
        return Promise.reject(err);
      }
      const config = err.config;
      return api.post("/auth/refresh", { token: cookies["auth.r"] }).then((res: AxiosResponse<{token: string}>) => {
        setLogin({tok: res.data.token});
        // issue here: https://github.com/axios/axios/issues/5089
        config.headers = JSON.parse(JSON.stringify(config.headers || {})) as RawAxiosRequestHeaders;

        config.headers["Authorization"] = `Bearer ${res.data.token}`;
        return api.request(config);
      }).catch((e: AxiosError) => {
        if (e.name === "CanceledError") {
          return Promise.reject(err);
        }
        resetLogin();
        dispatch(setAuthed(false));
        return Promise.reject(err);
      });
    });
  return provider(api);
}
