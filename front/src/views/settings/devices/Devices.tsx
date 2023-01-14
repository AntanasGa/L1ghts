import { AxiosError } from "axios";
import Loader, { Stage } from "components/Loader/Loader";
import { useAppDispatch, useAppSelector } from "hooks";
import React, { MouseEvent, useContext, useEffect, useState } from "react";
import { setDevices } from "store/devices";
import { ApiContext } from "utils/api";

export default function Devices() {
  const api = useContext(ApiContext);
  const devices = useAppSelector(store => store.devices.devices);
  const [loading, setLoading] = useState(!devices);
  const dispatch = useAppDispatch();

  useEffect(() => {
    if (!api) {
      return;
    }
    if (devices) {
      setLoading(false);
      return;
    }
    const cancel = api.cancelable();
    api.devices.get(cancel.token)
      .then(res => {
        setLoading(false);
        dispatch(setDevices(res.data));
      })
      .catch((err: AxiosError) => {
        if (err.name === "CanceledError" || err.status === 401 || err.response?.status === 401) {
          return;
        }
        setLoading(false);
      });
    return () => {
      cancel.cancel();
    };
  }, [api, dispatch, devices]);

  function onRefreshClick(e: MouseEvent<HTMLButtonElement>) {
    if (loading) {
      return;
    }
    setLoading(true);
    api?.devices.refresh()
      .then((res) => {
        setLoading(false);
        dispatch(setDevices(res.data));
      })
      .catch((e: AxiosError) => {
        setLoading(false);
        // FIXME: missing toast
      });
  }
  return (
    <div className="flex flex-col">
      <div className="flex">
        <h1 className="font-bold text-4xl">Devices</h1>
        { loading &&
        <Loader size={10} stage={Stage.active} />
        }
      </div>
      <button className="btn primary" type="button" onClick={ onRefreshClick }>Refresh devices</button>
      <hr />
      <table className="table-fixed text-left">
        <thead>
          <tr className="border-b-4">
            <th>Address</th>
            <th>End points</th>
          </tr>
        </thead>
        <tbody>
          { devices && devices?.length > 0
            ?
            devices.map((dev) => (
              <tr key={dev.id} className="border-b">
                <td>0x{dev.adr.toString().padStart(2, "0")}</td>
                <td>{dev.endpoint_count}</td>
              </tr>
            ))
            :
            <tr>
              <td colSpan={2}>No devices detected</td>
            </tr>
          }
        </tbody>
      </table>
    </div>
  );
}
