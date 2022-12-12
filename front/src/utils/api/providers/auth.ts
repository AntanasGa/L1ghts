import { AxiosInstance, AxiosPromise, CancelToken } from "axios";
import { getCookies } from "utils/cookies";
import { AuthPost, AuthReqPost } from "../types.api";

function auth(axios: AxiosInstance) {
  return {
    login: function (auth: AuthReqPost, cancelToken?: CancelToken): AxiosPromise<AuthPost> {
      return axios.post("/auth", auth, { cancelToken });
    },
    logout: function (cancelToken?: CancelToken): AxiosPromise {
      const cookies = getCookies();
      if (!cookies["auth.r"]) {
        return Promise.reject({ message: "No refresh token found" });
      }
      return axios.delete("/auth", { cancelToken, data: { token: cookies["auth.r"] } });
    },
  };
}

export default auth;
