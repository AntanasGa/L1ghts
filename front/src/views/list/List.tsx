import { AxiosError } from "axios";
import EditEndpoint from "components/EditEndpoint/EditEndpoint";
import Loader, { Stage } from "components/Loader/Loader";
import Modal from "components/Modal/Modal";
import { useAppDispatch, useAppSelector } from "hooks";
import React, { ChangeEvent, MouseEvent, useContext, useEffect, useMemo, useState } from "react";
import { setDevices } from "store/devices";
import { setPoints } from "store/points";
import { ApiContext } from "utils/api";
import { MAX_INTENSITY } from "utils/variables";

export default function List() {
  const api = useContext(ApiContext);
  const devices = useAppSelector(state => state.devices.devices);
  const points = useAppSelector(state => state.points.points);
  const dispatch = useAppDispatch();

  const [holdActions, setHoldActions] = useState(false);
  const [allSelector, setAllSelector] = useState(false);
  const [modalActive, setModalActive] = useState(false);
  const [pointSelector, setPointSelector] = useState<{[k: number]: boolean}>({});
  const [editPoints, setEditPoints] = useState<number[]>([]);
  const disPoints = useMemo(
    () => {
      const devList: {[key: string]: number} = {};
      return devices && points && [...points]
        .map((v) => {
          devList[v.device_id] = devList[v.device_id] ? devList[v.device_id] + 1 : 1;
          return {
            ...v,
            device_id: (devices.find((dv) => dv.id === v.device_id)?.adr || 0).toString().padStart(2, "0"),
            i_device_id: v.device_id,
            // device_position counts from 0
            device_position: v.device_position + 1,
          };
        })
        .sort((a, b) => {
          if (a.i_device_id === b.i_device_id) {
            return a.device_position - b.device_position;
          }
          return a.i_device_id - b.i_device_id;
        });
    },
    [devices, points]
  );

  useEffect(() => {
    if (!api) {
      return;
    }
    setHoldActions(!devices || !points);
    const cancelDevices = api.cancelable();
    if (!devices) {
      api.devices.get(cancelDevices.token)
        .then((res) => {
          dispatch(setDevices(res.data));
        })
        .catch((err: AxiosError) => {
          if (err.name === "CanceledError" || err.status === 401 || err.response?.status === 401) {
            return;
          }
          console.log(err);
        })
        .finally(() => setHoldActions(false));
    }
    const cancelPoints = api.cancelable();
    if (!points) {
      api.points.get(cancelPoints.token)
        .then((res) => {
          dispatch(setPoints(res.data));
        })
        .catch((err: AxiosError) => {
          if (err.name === "CanceledError" || err.status === 401 || err.response?.status === 401) {
            return;
          }
          console.log(err);
        })
        .finally(() => setHoldActions(false));
    }
    return () => {
      setHoldActions(false);
      cancelDevices.cancel();
      cancelPoints.cancel();
    };
  }, [api, dispatch, devices, points]);

  function updatePointSelector(e: ChangeEvent<HTMLInputElement>, pointId: number) {
    const update = {...pointSelector};
    if (update[pointId]) {
      delete update[pointId];
    } else {
      update[pointId] = true;
    }
    setAllSelector(Object.values(update).length === (points?.length || 0));
    setPointSelector(update);
  }

  function setActive(_: MouseEvent<HTMLButtonElement>, setTo: boolean) {
    if (!points || !api || holdActions) {
      return;
    }
    setHoldActions(true);
    const update = points?.filter(v => v.id in pointSelector).map(v => ({ ...v, active: setTo }));
    api.points.update(update)
      .then(res => {
        dispatch(setPoints(res.data));
      })
      .catch(err => {
      // FIXME: add toasts
        console.log(err);
      })
      .finally(() => setHoldActions(false));
  }

  function handleModalClose() {
    setModalActive(false);
    setEditPoints([]);
  }

  function handleIdentifyClick(id: number) {
    api?.points.identify(id).catch((e) => {
      // FIXME: missing toast
      console.log({ id });
      console.log(e);
    });
  }

  return (
    <div className="flex flex-col place-content-center content-center min-h-screen">
      <Modal
        portalKey="List-Update-dialog"
        active={ modalActive }
        onClose={(_) => handleModalClose()}
        label="Edit light points"
      >
        <EditEndpoint
          ids={ editPoints }
          close={ handleModalClose }
        />
      </Modal>
      <div className="card self-center mt-2 mb-[6.5rem] flex flex-col gap-4 container mx-auto">
        <div className="flex">
          <h1 className="font-bold text-4xl">Endpoint list</h1>
          { (!disPoints || holdActions) &&
            <Loader size={10} stage={Stage.active} />
          }
        </div>
        <table className="table-fixed text-left" aria-label="Device endpoints">
          <thead className="sticky top-0 card">
            <tr aria-label="table headers">
              <th aria-label="Select or deselect all endpoints">
                <label htmlFor="endpoint-list-select-all" className="opacity-0 absolute select-none">Select or deselect all endpoints</label>
                <input
                  id="endpoint-list-select-all"
                  type="checkbox"
                  checked={ allSelector }
                  onChange={ (e) => {
                    setAllSelector(e.target.checked);
                    setPointSelector(e.target.checked ? Object.fromEntries(disPoints?.map((v) => [v.id, true]) || []) : {} );
                  }}
                />
              </th>
              <th>Device</th>
              <th>Point id</th>
              <th>Tag</th>
              <th>Active</th>
              <th>Value</th>
              <th>Actions</th>
            </tr>
            { Object.keys(pointSelector).length > 0
            &&
              <tr>
                <th colSpan={7}>
                  <button
                    className="btn"
                    type="button"
                    onClick={ (_) => {
                      const pts = disPoints?.filter(v => !(v.id in pointSelector)).map(v => [v.id, true]) || [];
                      setPointSelector(
                        Object.fromEntries(
                          pts
                        )
                      );
                      setAllSelector(pts.length === (points?.length || 0));
                    }
                    }
                  >Invert selection</button>
                  <button className="ml-1 btn" type="button" onClick={(e) => setActive(e, true)}>Activate</button>
                  <button className="ml-1 btn" type="button" onClick={(e) => setActive(e, false)}>Deactivate</button>
                  <button
                    className="ml-1 btn"
                    type="button"
                    onClick={() => {
                      // not the best thing but it works
                      setEditPoints(Object.keys(pointSelector).map((e) => parseInt(e)));
                      setModalActive(true);
                    }}
                  >Edit</button>
                </th>
              </tr>
            }
          </thead>
          <tbody>
            { disPoints &&
            disPoints.map(p =>
              <tr key={ p.id } aria-label={ `Device 0x${p.device_id}, endpoint ${p.device_position}` }>
                <td aria-label="Select / deselect endpoint for editing"><input type="checkbox" checked={pointSelector[p.id] || false} onChange={(e) => updatePointSelector(e, p.id)}/></td>
                <td aria-label="Device">0x{ p.device_id }</td>
                <td aria-label="Endpoint">{ p.device_position }</td>
                <td aria-label="tag">{ p.tag || "-" }</td>
                <td aria-label="Currently active">{ p.active ? "Yes" : "No" }</td>
                <td aria-label="Light intensity">
                  <label
                    htmlFor={ `0x${ p.device_id }-${ p.tag || p.device_position }-pr` }
                  >{ p.val } / { MAX_INTENSITY }</label>
                  <br />
                  <progress
                    className="w-8 sm:w-16 md:w-32"
                    id={ `0x${ p.device_id }-${ p.tag || p.device_position }-pr` }
                    max={ MAX_INTENSITY }
                    value={ p.val }
                    aria-label={ `0x${ p.device_id } ${ p.tag || p.device_position } power ${ p.val } of ${ MAX_INTENSITY }` }
                  />
                </td>
                <td aria-label="Actions">
                  <button
                    className="btn primary"
                    type="button"
                    onClick={() => {
                      setEditPoints([p.id]);
                      setModalActive(true);
                    }}
                  >Edit</button>
                  <button
                    className="btn"
                    type="button"
                    onClick={ () => handleIdentifyClick(p.id) }
                  >Identify</button>
                </td>
              </tr>
            )
            }
          </tbody>
        </table>
        
      </div>
    </div>
  );
}
