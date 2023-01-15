import ax from "axios";
import { AxiosInstance, CancelTokenSource } from "axios";
import auth from "./providers/auth";
import devices from "./providers/devices";
import points from "./providers/points";
import presets from "./providers/presets";
import step from "./providers/step";

function methods(axios: AxiosInstance) {
  return {
    auth: auth(axios),
    devices: devices(axios),
    points: points(axios),
    presets: presets(axios),
    step: step(axios),
    cancelable: function(): CancelTokenSource {
      return ax.CancelToken.source();
    }
  };
}
export default methods;
