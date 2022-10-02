import { createSlice } from "@reduxjs/toolkit";

const initialState = {
  token: "",
  user: {},
};

export const authSlice = createSlice({
  name: "auth",
  initialState,
  reducers: {
    authenticate(state, action) {
      state.token = action.payload.token;
      state.user = action.payload.user;
    },
  },
});

export const { authenticate } = authSlice.actions;

export default authSlice.reducer;
