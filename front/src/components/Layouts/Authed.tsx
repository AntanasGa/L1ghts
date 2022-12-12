import React, { useContext, useEffect, useRef, useState } from "react";
import { Link, Outlet, useNavigate } from "react-router-dom";
import Template from "../Icons/Template";
import Presets from "../Icons/Presets";
import Settings from "../Icons/Settings";
import List from "../Icons/List";
import { useAppDispatch, useAppSelector } from "hooks";
import { ApiContext } from "utils/api";
import Modal from "components/Modal/Modal";
import EditPreset from "components/EditPreset/EditPreset";
import { setPresetsActive, setPresets } from "store/presets";
import { AxiosError } from "axios";
import Pencil from "components/Icons/Pencil";
import EyeDropper from "components/Icons/EyeDropper";
import { setPoints } from "store/points";

export default function Authed() {
  const navigate = useNavigate();
  const api = useContext(ApiContext);
  const dispatch = useAppDispatch();

  const mounted = useRef(false);
  const presetBox = useRef<null|HTMLLIElement>(null);
  const ignoreClick = useRef(false);

  const authed = useAppSelector(store => store.auth.authed);
  const presets = useAppSelector(store => store.presets.presets);
  const activePreset = useAppSelector(store => store.presets.active);
  
  const [presetActive, setPresetActive] = useState(false);
  const [presetDropdownActive, setPresetDropdownActive] = useState(false);
  const [presetId, setPresetId] = useState<undefined|number>(undefined);

  useEffect(() => {
    if (!authed && authed !== undefined) {
      navigate("/");
    }
  }, [authed, navigate]);

  useEffect(() => {
    if (!api) {
      return;
    }
    const token = api.cancelable();
    const activeToken = api.cancelable();
    api.presets.get(token.token)
      .then((e) => {
        dispatch(setPresets(e.data));
      })
      .catch((e: AxiosError) => {
        if (e.name === "CanceledError") {
          return;
        }
        console.error(e);
      });
    api.presets.active.get(activeToken.token)
      .then((r) => {
        dispatch(setPresetsActive(r.data.id));
      })
      .catch((e: AxiosError) => {
        if (e.name === "CanceledError") {
          return;
        }
        console.error(e);
      });
    return () => {
      token.cancel();
      activeToken.cancel();
    };
  }, [api, dispatch]);

  useEffect(() => {
    function exitPresets(e: MouseEvent | KeyboardEvent) {
      if (!presetDropdownActive || !presetBox.current) {
        return;
      }
      // this somehow happens the same call cycle as the element being created so this is a workaround
      if (ignoreClick.current) {
        ignoreClick.current = false;
        return;
      }
      if (e instanceof KeyboardEvent && e.key !== "esc") {
        return;
      }
      if (e instanceof MouseEvent) {
        const rect = presetBox.current.getBoundingClientRect();
        const clickInBox = rect.top <= e.clientY && rect.bottom >= e.clientY
          && rect.left <= e.clientX && rect.right >= e.clientX;
        if (clickInBox) {
          return;
        }
      }
      setPresetDropdownActive(false);
    }
    if (!mounted.current) {
      mounted.current = true;
      window.addEventListener("click", exitPresets);
      window.addEventListener("keypress", exitPresets);
    }
    return () => {
      mounted.current = false;
      window.removeEventListener("click", exitPresets);
      window.removeEventListener("keypress", exitPresets);
    };
  }, [presetDropdownActive]);


  function setPresetToActive(id: number) {
    api?.presets.active.update(id)
      .then((r) => {
        dispatch(setPresetsActive(r.data.id));
        api.points.get()
          .then((v) => {
            dispatch(setPoints(v.data));
          })
          .catch((e) => {
            if (e.name === "CanceledError") {
              return;
            }
          });
      })
      .catch((e: AxiosError) => {
        const status = e.status || e.response?.status;
        if (status === 429) {
          // TODO: toast for server being busy
        }
        if (status === 409) {
          // TODO: toast for does not exist
        }
        console.log(e);
      });
  }

  function grabPresetCurrentSetup(id: number) {
    api?.presets.update_points(id)
      .then((v) => {
        dispatch(setPresetsActive(v.data.id));
      })
      .catch();
  }

  function callEditPreset(id: number) {
    setPresetId(id);
    setPresetActive(true);
  }

  function onPresetModalClose() {
    setPresetActive(false);
    setPresetId(undefined);
  }

  return (
    <>
      <Outlet />
      <Modal
        portalKey="preset-edit"
        label="preset modal"
        active={ presetActive }
        onClose={() => onPresetModalClose() }
      >
        <EditPreset id={ presetId } onClose={() => onPresetModalClose() } />
      </Modal>
      <nav className="fixed bottom-0 left-0 right-0 flex" aria-label="Logged in navigation">
        <div className="card p-0 rounded m-3 mx-auto flex">
          <Link to="/dashboard" className="p-2" aria-label="Dashboard">
            <Template classNameArr={["w-16", "h-16"]} />
          </Link>
          <div aria-label="preset items">
            <button
              type="button"
              className="p-2"
              aria-label="Presets"
              onClick={ (_) => {
                setPresetDropdownActive((v) => !v);
                ignoreClick.current = true;
              }}
              
            >
              <Presets classNameArr={["w-16", "h-16"]} />
            </button>
            { presetDropdownActive &&
              <ul className="absolute bottom-[5.5rem] card" aria-label="Preset list">
                { presets &&
                  presets.map((e) =>
                    <li key={ e.id } className="flex gap-1 justify-between" ref={ presetBox }>
                      { activePreset === e.id && 
                      <>&gt;</>
                      }
                      <button
                        type="button"
                        onClick={ (_) => setPresetToActive(e.id) }
                      >{ e.preset_name }</button>
                      <button
                        type="button"
                        onClick={ (_) => callEditPreset(e.id) }
                      ><Pencil classNameArr={["w-6", "h-6"]} /></button>
                      <button
                        type="button"
                        onClick={ (_) => grabPresetCurrentSetup(e.id) }
                      ><EyeDropper classNameArr={["w-6", "h-6"]} /></button>
                    </li>
                  )
                }
                <li><button type="button" className="btn primary" onClick={ () => setPresetActive(true) }>Save current</button></li>
              </ul>
            }
          </div>
          <Link to="/list" className="p-2" aria-label="Light endpoints">
            <List classNameArr={["w-16", "h-16"]} />
          </Link>
          <Link to="/settings" className="p-2" aria-label="Settings">
            <Settings classNameArr={["w-16", "h-16"]} />
          </Link>
        </div>
      </nav>
    </>
  );
}
