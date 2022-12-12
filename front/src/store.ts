import { configureStore } from "@reduxjs/toolkit";
import devices from "store/devices";
import points from "store/points";
import presets from "store/presets";
import auth from "./store/auth";

const store = configureStore({
  reducer: {
    auth,
    devices,
    points,
    presets,
  }
});

export type RootState = ReturnType<typeof store.getState>;

export type AppDispatch = typeof store.dispatch;

export default store;
