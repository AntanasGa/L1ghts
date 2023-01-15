import { AxiosError } from "axios";
import { useAppSelector } from "hooks";
import React, { FormEvent, useContext, useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import { ApiContext } from "utils/api";

export default function Settings() {
  const navigate = useNavigate();
  const api = useContext(ApiContext);

  const [errs, setErrs] = useState({setup_key: "", user_name: "", password: ""});
  const [loading, setLoading] = useState(false);

  const loggedIn = useAppSelector(state => state.auth.authed);

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
    if (!["setup_key", "new_user_name", "new_password"].every((v) => data.has(v))) {
      setErrs({
        setup_key: data.has("setup_key") ? "" : "Setup key missing",
        user_name: data.has("new_user_name") ? "" : "Username missing",
        password: data.has("new_password") ? "" : "Password missing",
      });
      setLoading(false);
      return;
    }
    const formFields = {
      setup_key: data.get("setup_key"),
      user_name: data.get("new_user_name"),
      password: data.get("new_password"),
    };
    console.log({formFields});
    if (!formFields.setup_key || !formFields.user_name || !formFields.password) {
      console.log("somethings empty");
      console.log(JSON.parse(JSON.stringify(formFields)));
      setErrs({
        setup_key: formFields.setup_key ? "" : "Setup key empty",
        user_name: formFields.user_name ? "" : "Username empty",
        password: formFields.password ? "" : "Password empty",
      });
      setLoading(false);
      return;
    }
    if (
      formFields.setup_key instanceof File
      || formFields.user_name instanceof File
      || formFields.password instanceof File
    ) {
      setErrs({
        setup_key: formFields.setup_key instanceof File ? "Setup key input was messed with" : "",
        user_name: formFields.user_name instanceof File ? "Username input was messed with" : "",
        password: formFields.password instanceof File ? "Password input was messed with" : "",
      });
      setLoading(false);
      return;
    }
    api.step.post(formFields.setup_key, formFields.user_name, formFields.password).then((res) => {
      navigate("/");
    }).catch((err: AxiosError) => {
      let foundStatus = false;
      switch (err.status || err.response?.status) {
      case 400:
        setErrs({
          setup_key: "",
          user_name: (formFields.user_name?.toString() || "").length > 8 ? "" : "Username too short", 
          password: (formFields.password?.toString() || "").length > 8 ? "" : "Password too short", 
        });
        foundStatus = true;
        break;
      case 401:
        setErrs({
          setup_key: "Incorrect key",
          user_name: "",
          password: "",
        });
        foundStatus = true;
        break;
      case 403:
        setErrs({
          setup_key: "Already used",
          user_name: "",
          password: "",
        });
        foundStatus = true;
        break;
      }
      if (!foundStatus) {
        console.log(err);
      }

    }).finally(() => setLoading(false));
  }

  return (
    <div className="flex items-center place-content-center h-screen">
      <div className="flex flex-col items-center card self-center">
        <h1 className="text-6xl">Setup</h1>
        <form className="flex flex-col pt-8" onSubmit={handleOnSubmit}>
          {errs.user_name && <p className="text-red-600">{errs.user_name}</p>}
          {errs.password && <p className="text-red-600">{errs.password}</p>}
          {errs.setup_key && <p className="text-red-600">{errs.setup_key}</p>}

          <label htmlFor="setup_key">Setup key:</label>
          <input
            type="text"
            name="setup_key"
            id="setup_key"
            autoComplete="off"
            disabled={loading}
            onInput={() => { setErrs((e) => ({ user_name: e.user_name, password: e.password, setup_key: "" })); }}
          />

          <label htmlFor="new_user_name">Username:</label>
          <input
            type="text"
            name="new_user_name"
            id="new_user_name"
            autoComplete="username"
            disabled={loading}
            onInput={() => { setErrs((e) => ({ user_name: "", password: e.password, setup_key: e.setup_key })); }}
          />
          <label htmlFor="new_password">Password:</label>
          <input
            type="password"
            name="new_password"
            id="new_password"
            autoComplete="new-password"
            disabled={loading}
            onInput={() => { setErrs((e) => ({ user_name: e.user_name, password: "", setup_key: e.setup_key })); }}
          />
          <input
            type="submit"
            value="Initialize"
            className="btn primary mt-2"
            disabled={loading}
          />
        </form>
      </div>
    </div>
  );
}