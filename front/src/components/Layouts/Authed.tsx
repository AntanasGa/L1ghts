import React, {useCallback, useContext, useEffect, useMemo, useRef, useState } from "react";
import type { MouseEvent as MouseEv} from "react";
import { Link, Outlet, useNavigate } from "react-router-dom";
import Template from "../Icons/Template";
import Presets from "../Icons/Presets";
import Settings from "../Icons/Settings";
import List from "../Icons/List";
import { useAppDispatch, useAppSelector, useRefedState } from "hooks";
import { ApiContext } from "utils/api";
import Modal from "components/Modal/Modal";
import EditPreset from "components/EditPreset/EditPreset";
import { setPresetsActive, setPresets } from "store/presets";
import { AxiosError } from "axios";
import Pencil from "components/Icons/Pencil";
import EyeDropper from "components/Icons/EyeDropper";
import { setPoints } from "store/points";
import { setJustLoggedIn } from "store/auth";

export default function Authed() {
  const navigate = useNavigate();
  const api = useContext(ApiContext);
  const dispatch = useAppDispatch();

  const authed = useAppSelector(store => store.auth.authed);
  const presets = useAppSelector(store => store.presets.presets);
  const activePreset = useAppSelector(store => store.presets.active);
  const justLoggedIn = useAppSelector(store => store.auth.justLoggedIn);

  const [presetActive, setPresetActive] = useState(false);
  const [presetDropdownActiveRef, presetDropdownActive, setPresetDropdownActive] = useRefedState(false);
  const [presetId, setPresetId] = useState<undefined|number>(undefined);
  const [navOverflow, setNavOverflow] = useState(justLoggedIn);

  const didMount = useRef(false);
  const presetBox = useRef<null|HTMLUListElement>(null);
  const ignoreClick = useRef(false);

  const menuClasses = useMemo(() => {
    const result = ["card p-0 rounded mb-3 mx-auto flex ease-in duration-150"];
    if (justLoggedIn !== false) {
      result.push("mb-[-10rem]");
    }
    return result;
  }, [justLoggedIn]);

  const navClasses = useMemo(() => {
    const result = ["fixed bottom-0 left-0 right-0 flex"];
    if (navOverflow !== false) {
      result.push("overflow-hidden");
    }
    return result;
  }, [navOverflow]);

  function callEditPreset(id: number) {
    setPresetId(id);
    setPresetActive(true);
  }

  function onPresetModalClose() {
    setPresetActive(false);
    setPresetId(undefined);
  }

  useEffect(() => {
    if (!authed && authed !== undefined) {
      navigate("/");
    }
  }, [authed, navigate]);

  useEffect(() => {
    if (!api) {
      return;
    }
    let cancelGroup: (() => void) | undefined;
    if (!didMount.current) {
      didMount.current = true;
      window.addEventListener("click", exitPresets);
      window.addEventListener("keypress", exitPresets);
      dispatch(setJustLoggedIn(false));

      cancelGroup = loadPresets();
    }
    return () => {
      didMount.current = false;
      window.removeEventListener("click", exitPresets);
      window.removeEventListener("keypress", exitPresets);
      cancelGroup?.();
    };
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api, dispatch]);

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

  function loadPresets() {
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
  }

  const exitPresets = useCallback((e: MouseEvent | KeyboardEvent) => {
    if (!presetDropdownActiveRef.current || !presetBox.current) {
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
  }, [presetDropdownActiveRef, setPresetDropdownActive, presetBox]);

  const onPresetListClick = /*useCallback(*/(e: MouseEv<HTMLButtonElement>) => {
    e.preventDefault();
    setPresetDropdownActive((v) => !v);
    ignoreClick.current = true;
  }/*, [presetDropdownActive])*/;

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
      <nav className={ navClasses.join(" ") } onTransitionEnd={ (ev) => setNavOverflow(false) } aria-label="Logged in navigation">
        <div className={ menuClasses.join(" ") }>
          <Link to="/dashboard" className="p-2" aria-label="Dashboard">
            <Template classNameArr={["w-16", "h-16"]} />
          </Link>
          <div aria-label="preset items">
            <button
              type="button"
              className="p-2"
              aria-label="Presets"
              onClick={ onPresetListClick }
              
            >
              <Presets classNameArr={["w-16", "h-16"]} />
            </button>
            { presetDropdownActive &&
              <ul className="absolute bottom-[5.5rem] card" aria-label="Preset list" ref={ presetBox }>
                { presets &&
                  presets.map((e) =>
                    <li key={ e.id } className="flex gap-1 justify-between">
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
