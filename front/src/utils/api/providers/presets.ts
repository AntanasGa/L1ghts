import { AxiosInstance, AxiosPromise, CancelToken } from "axios";
import { NewPresets, Presets, QueryById } from "../types.api";

function presets(axios: AxiosInstance) {
  return {
    get: function (cancelToken?: CancelToken): AxiosPromise<Presets[]> {
      return axios.get("/presets", { cancelToken });
    },
    create: function (preset: NewPresets, cancelToken?: CancelToken): AxiosPromise<Presets[]> {
      return axios.post("/presets", preset, { cancelToken });
    },
    update: function (preset: Presets, cancelToken?: CancelToken): AxiosPromise<Presets[]> {
      return axios.put("/presets", preset, { cancelToken });
    },
    update_points: function (id: number, cancelToken?: CancelToken): AxiosPromise<QueryById> {
      return axios.put("/presets/points", { id }, { cancelToken });
    },
    active: {
      get: function (cancelToken?: CancelToken): AxiosPromise<QueryById> {
        return axios.get("/presets/active", { cancelToken });
      },
      update: function (id: number, cancelToken?: CancelToken): AxiosPromise<QueryById> {
        return axios.put("/presets/active", { id }, { cancelToken });
      }
    },
  };
}

export default presets;