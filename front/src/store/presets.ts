import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { Presets } from "utils/api/types.api";

interface PresetState {
  presets?: Presets[],
  active?: number,
}

const initialState: PresetState = {
  presets: undefined,
  active: undefined,
};

export const authSlice = createSlice({
  name: "presets",
  initialState,
  reducers: {
    setPresets: (state, action: PayloadAction<Presets[]>) => {
      state.presets = action.payload;
    },
    setPresetsActive: (state, action: PayloadAction<number>) => {
      state.active = action.payload;
    },
  },
});

export const { setPresets, setPresetsActive } = authSlice.actions;

export default authSlice.reducer;
