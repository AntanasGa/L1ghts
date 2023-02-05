import { createSlice, PayloadAction } from "@reduxjs/toolkit";

interface AuthState {
  authed?: boolean,
  justLoggedIn?: boolean,
}

const initialState: AuthState = {
  authed: undefined,
  justLoggedIn: undefined,
};

export const authSlice = createSlice({
  name: "auth",
  initialState,
  reducers: {
    setAuthed: (state, action: PayloadAction<boolean>) => {
      state.authed = action.payload;
    },
    setJustLoggedIn: (state, action: PayloadAction<boolean | undefined>) => {
      console.log("what", action);
      state.justLoggedIn = action.payload;
    },
  },
});

export const { setAuthed, setJustLoggedIn } = authSlice.actions;

export default authSlice.reducer;
