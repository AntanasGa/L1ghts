import { AxiosInstance, AxiosPromise, CancelToken } from "axios";
import { StepGet } from "../types.api";

function step(axios: AxiosInstance) {
  return {
    get: function (cancelToken?: CancelToken): AxiosPromise<StepGet> {
      return axios.get("/step", {cancelToken});
    },
    post: function (key: string, user_name: string, password: string, cancelToken?: CancelToken): AxiosPromise {
      return axios.post("/step", { key, user: { user_name, password }}, {cancelToken});
    },
  };
}

export default step;
