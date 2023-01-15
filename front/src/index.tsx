import React from "react";
import ReactDOM from "react-dom/client";
import { BrowserRouter, Routes, Route } from "react-router-dom";
import "./style/index.sass";
import reportWebVitals from "./reportWebVitals";
import Index from "./views/index/Index";
import Login from "./views/login/Login";
import Dashboard from "./views/dashboard/Dashboard";
import Authed from "./components/Layouts/Authed";
import List from "./views/list/List";
import Settings from "./views/settings/Settings";
import CatchAll from "./views/catchAll/CatchAll";
import { Provider } from "react-redux";
import store from "store";
import Api from "components/Api";
import AuthedSettings from "components/Layouts/AuthedSettings";
import Devices from "views/settings/devices/Devices";
import Setup from "views/setup/Setup";

const root = ReactDOM.createRoot(
  document.getElementById("root") as HTMLElement
);


root.render(
  <React.StrictMode>
    <Provider store={ store }>
      <BrowserRouter>
        <Routes>
          <Route element={ <Api /> }>
            <Route path="/" element={ <Index /> } />
            <Route path="/setup" element={ <Setup /> } />
            <Route path="/login" element={ <Login /> } />
            <Route element={ <Authed /> }>
              <Route path="/dashboard" element={ <Dashboard /> } />
              <Route path="/list" element={ <List/> }/>
              <Route element={ <AuthedSettings /> }>
                <Route path="/settings" element={ <Settings /> } />
                <Route path="/settings/devices" element={ <Devices /> } />
              </Route>
            </Route>
            <Route path="*" element={ <CatchAll /> } />
          </Route>
        </Routes>
      </BrowserRouter>
    </Provider>
  </React.StrictMode>
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals
reportWebVitals();
