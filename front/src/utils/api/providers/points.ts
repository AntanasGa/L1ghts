import { AxiosInstance, AxiosPromise, CancelToken } from "axios";
import { Points, QueryById } from "../types.api";

function points(axios: AxiosInstance) {
  return {
    get: function(cancelToken?: CancelToken): AxiosPromise<Points[]> {
      return axios.get("/points", { cancelToken });
    },
    update: function(points: Points[], cancelToken?: CancelToken): AxiosPromise<Points[]> {
      return axios.put("/points", points, { cancelToken });
    },
    identify: function(id: number, cancelToken?: CancelToken): AxiosPromise<QueryById> {
      return axios.post("/points/identify", { id }, { cancelToken });
    },
  };
}

export default points;
