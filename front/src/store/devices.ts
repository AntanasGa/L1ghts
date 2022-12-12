import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { Devices } from "utils/api/types.api";

interface DeviceState {
  devices?: Devices[],
}

const initialState: DeviceState = {
  devices: undefined,
};

export const deviceSlice = createSlice({
  name: "device",
  initialState,
  reducers: {
    setDevices: (state, action: PayloadAction<Devices[]>) => {
      state.devices = action.payload;
    },
  },
});

export const { setDevices } = deviceSlice.actions;

export default deviceSlice.reducer;
