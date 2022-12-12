import ax from "axios";
import { AxiosInstance, AxiosPromise, CancelToken, CancelTokenSource } from "axios";
import auth from "./providers/auth";
import devices from "./providers/devices";
import points from "./providers/points";
import presets from "./providers/presets";
import { StepGet } from "./types.api";

function methods(axios: AxiosInstance) {
  return {
    auth: auth(axios),
    devices: devices(axios),
    points: points(axios),
    presets: presets(axios),
    step: function (cancelToken?: CancelToken): AxiosPromise<StepGet> {
      return axios.get("/step", {cancelToken});
    },
    cancelable: function(): CancelTokenSource {
      return ax.CancelToken.source();
    }
  };
}
export default methods;
