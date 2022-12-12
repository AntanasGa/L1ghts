import { AxiosError } from "axios";
import Loader, { Stage } from "components/Loader/Loader";
import { useAppDispatch, useAppSelector } from "hooks";
import React, { FormEvent, useContext, useEffect, useRef, useState } from "react";
import { setPresets } from "store/presets";
import { ApiContext } from "utils/api";
import { NewPresets } from "utils/api/types.api";

interface EditPresetProps {
  id?: number,
  onClose?: () => void,
}

export default function EditPreset({ id, onClose }: EditPresetProps) {
  const api = useContext(ApiContext);
  const dispatch = useAppDispatch();

  const presets = useAppSelector(state => state.presets.presets);

  const submitRef = useRef<HTMLButtonElement|null>(null);
  const transactionComplete = useRef(false);

  const [loading, setLoading] = useState(false);
  const [presetName, setPresetName] = useState("");
  const [favorite, setFavorite] = useState(false);
  const [formErrors, setFormErrors] = useState({
    name: "",
    exists: "",
  });

  const errorMap: {[key: string]: string} = {
    name: "Name",
  };

  useEffect(() => {
    if (!id || !presets) {
      return;
    }
    const preset = presets.find(v => v.id === id);
    console.log(presets, preset, id);
    if (!preset) {
      return;
    }
    setPresetName(preset.preset_name);
    setFavorite(preset.favorite);
  }, [presets, id]);

  function handleSubmit(e: FormEvent<HTMLFormElement>) {
    if (transactionComplete.current) {
      onClose?.();
      return true;
    }
    e.preventDefault();
    if (!presets || !api) {
      return false;
    }
    const setErrors = {
      name: (!presetName.trim() && "No name set") || "",
      exists: "",
    };
    if (Object.values(setErrors).some(v => v)) {
      setFormErrors(setErrors);
      return false;
    }
    transactionComplete.current = true;
    setLoading(true);
    const preset: NewPresets = {
      preset_name: presetName,
      favorite,
      icon: null
    };
    if (!id) {
      api.presets.create(preset)
        .then((v) => {
          dispatch(setPresets(v.data));
          window.requestAnimationFrame(() => {
            submitRef.current?.click();
          });
        })
        .catch((e: AxiosError) => {
          const responseCode = e.response?.status || e.status || -1;
          if (responseCode === 409) {
            transactionComplete.current = false;
            setFormErrors((v) => ({ ...v, exists: "Point values must be unique" }));
            return false;
          }
          console.log(e);
        })
        .finally(() => {
          setLoading(false);
        });
    } else {
      api.presets.update({ ...preset, id })
        .then((v) => {
          dispatch(setPresets(v.data));
          window.requestAnimationFrame(() => {
            submitRef.current?.click();
          });
        })
        .catch((e: AxiosError) => {
          const responseCode = e.response?.status || e.status || -1;
          if (responseCode === 409) {
            transactionComplete.current = false;
            setFormErrors((v) => ({ ...v, exists: "Point values must be unique" }));
            return false;
          }
          console.log(e);
        });
    }
  }

  return (
    <form method="dialog" onSubmit={ handleSubmit }>
      <h1 className="font-bold text-4xl">{ id === undefined ? "Create preset" : "Edit preset"}</h1>
      { loading &&
        <Loader size={ 10 } stage={ Stage.active } />
      }
      <div className="flex flex-col mb-1">
        {
          Object.entries(formErrors)
            .filter(([_, v]) => v)
            .map(([k, v]) =>
              <p key={ k } className="text-red-600">
                { k in errorMap &&
                  <><b>{errorMap[k] || ""}:</b><br /></>
                }
                { v }
              </p>
            )
        }
      </div>
      <div className="flex flex-col my-1">
        <label htmlFor="edit-preset-name">Name</label>
        <input
          type="text"
          name="name"
          id="edit-preset-name"
          value={ presetName }
          onInput={ (e) => {
            setFormErrors((v) => ({...v, name: ""}));
            setPresetName(e.currentTarget.value);
          }}
        />
      </div>
      <div className="flex flex-col mb-1">
        <label htmlFor="edit-preset-favorite">Favorite</label>
        <input
          type="checkbox"
          name="favorite"
          id="edit-preset-favorite"
          checked={ favorite }
          onChange={ (e) => setFavorite(e.currentTarget.checked) }
        />
      </div>
      {/* TODO: add icon upload */}
      <div className="flex justify-between mt-4">
        <button value="cancel" onClick={ onClose } className="btn">Cancel</button>
        <button type="submit" className="btn primary" ref={ submitRef }>Save</button>
      </div>
    </form>
  );
}
