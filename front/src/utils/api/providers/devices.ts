import { AxiosInstance, AxiosPromise, CancelToken } from "axios";
import { Devices } from "../types.api";

function devices(axios: AxiosInstance) {
  return {
    get: function (cancelToken?: CancelToken): AxiosPromise<Devices[]> {
      return axios.get("/devices", {cancelToken});
    },
    refresh: function (cancelToken?: CancelToken): AxiosPromise<Devices[]> {
      return axios.post("/devices", { cancelToken });
    }
  };
}

export default devices;
