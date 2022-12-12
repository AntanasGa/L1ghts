import { createSlice, PayloadAction } from "@reduxjs/toolkit";
import { Points } from "utils/api/types.api";

interface PointState {
  points?: Points[],
}

const initialState: PointState = {
  points: undefined,
};

export const pointSlice = createSlice({
  name: "points",
  initialState,
  reducers: {
    setPoints: (state, action: PayloadAction<Points[]>) => {
      state.points = action.payload;
    },
  },
});

export const { setPoints } = pointSlice.actions;

export default pointSlice.reducer;
