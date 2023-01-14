import { AxiosError } from "axios";
import LightPointEditorShowcase from "components/LightPointEditorShowcase/LightPointEditorShowcase";
import Loader, { Stage } from "components/Loader/Loader";
import { useAppDispatch, useAppSelector } from "hooks";
import React, { FormEvent, useContext, useEffect, useRef, useState } from "react";
import { setPoints } from "store/points";
import { setPresetsActive } from "store/presets";
import { ApiContext } from "utils/api";
import { UpdatePoints } from "utils/api/types.api";
import { MAX_INTENSITY } from "utils/variables";

export interface EditEndpointProps {
  ids: number[],
  close: () => void,
}

export default function EditEndpoint({close, ids}: EditEndpointProps) {
  const api = useContext(ApiContext);
  const dispatch = useAppDispatch();

  const points = useAppSelector(state => state.points.points);

  const submitRef = useRef<HTMLButtonElement|null>(null);

  const [transactionComplete, setTransactionComplete] = useState(false);
  const [loader, setLoader] = useState(false);
  const [tag, setTag] = useState("");
  const [active, setActive] = useState(false);
  const [intensity, setIntensity] = useState(0);
  const [width, setWidth] = useState(0);
  const [height, setHeight] = useState(0);
  const [rotation, setRotation] = useState(0);
  const [watts, setWatts] = useState(0);
  const [errs, setErrs] = useState({
    val: "",
    width: "",
    height: "",
    rotation: "",
    watts: "",
  });

  // once again we can thank typescript :)
  const errorMap: {[key: string]: string} = {
    val: "Light intensity",
    width: "Width",
    height: "Height",
    rotation: "Rotation",
    watts: "Max watts",
  };

  useEffect(() => {
    const assign = {
      tag: "",
      active: false,
      val: 0,
      width: 0,
      height: 0,
      rotation: 0,
      watts: 0,
    };
    if (ids.length === 1) {
      const point = points?.find((e) => e.id === ids[0]);
      if (!point) {
        return;
      }
      assign.tag = point.tag || "";
      assign.active = point.active;
      assign.val = point.val;
      assign.width = point.width;
      assign.height = point.height;
      assign.rotation = point.rotation;
      assign.watts = point.watts;
    }
    setTag(assign.tag);
    setActive(assign.active);
    setIntensity(assign.val);
    setWidth(assign.width);
    setHeight(assign.height);
    setRotation(assign.rotation);
  }, [points, ids]);


  function handleSubmit(e: FormEvent<HTMLFormElement>) {
    if (transactionComplete) {
      close();
      return true;
    }
    e.preventDefault();
    if (!points || !api) {
      return false;
    }
    setTransactionComplete(true);
    const errorHandles = {
      val: (isNaN(intensity) && "Not a number")
      || (intensity < 0 && "Can't be a negative number")
      || (intensity > MAX_INTENSITY && `Can't exceed ${MAX_INTENSITY}`) 
      || "",
      width: (isNaN(width) && "Not a number")
      || (width < 0 && "can't be a negative number")
      || "",
      height: (isNaN(height) && "Not a number")
      || (height < 0 && "can't be a negative number")
      || "",
      rotation: (isNaN(rotation) && "Not a number")
      || (rotation < 0 && "Can't be a negative number")
      || (rotation > 360 && "Can't exceed 360") 
      || "",
      watts: (isNaN(watts) && "Not a number")
      || (watts < 0 && "can't be a negative number")
      || "",
    };
    if (Object.values(errorHandles).some((v) => v !== "")) {
      setErrs(errorHandles);
      return false;
    }

    if ([intensity, width, height, rotation].some((v) => isNaN(v) || v < 0)) {
      // handle number
      return false;
    }
    const updateWithUndefined: UpdatePoints[] = [...ids].map((id) => ({
      id,
      val: intensity,
      width,
      height,
      x: points[id].x || 0,
      y: points[id].y || 0,
      rotation,
      watts,
      active,
      tag, 
    }));
    const update = updateWithUndefined.filter(entry => typeof entry != "undefined");
    if (updateWithUndefined.length !== update.length) {
      return;
    }
    setLoader(true);
    api.points.update(update)
      .then((res) => {
        dispatch(setPoints(res.data));
        dispatch(setPresetsActive(-1));
        // cant create click event on same frame as user click
        window.requestAnimationFrame(() => {
          submitRef.current?.click();
        });
      })
      .catch((err: AxiosError) => {
        console.log(err);
      })
      .finally(() => {
        setLoader(false);
      });
    return false;
  }

  return (
    <form method="dialog" onSubmit={ handleSubmit }>
      <h1 className="font-bold text-4xl">Edit Endpoint{ ids.length > 1 && "s" }</h1>
      { loader &&
        <Loader size={ 10 } stage={ Stage.active } />
      }
      <div className="flex mb-1">
        {
          Object.entries(errs)
            .filter(([_, v]) => v)
            .map(([k, v]) => <p key={ k } className="text-red-600"><b>{(k in errorMap && errorMap[k]) || ""}:</b><br/> {v}</p>)
        }
      </div>
      <div className="flex flex-col mb-1">
        <label htmlFor="edit-endpoints-tag" className="text-sm">Tag</label>
        <input id="edit-endpoints-tag" type="text" name="tag" value={ tag } onInput={ (e) => setTag(e.currentTarget.value.trim()) } />
      </div>
      <div className="flex flex-col mb-1">
        <label htmlFor="edit-endpoints-active">Active</label>
        <input id="edit-endpoints-active" type="checkbox" name="active" checked={ active } onChange={ (e) => setActive(e.target.checked) } />
      </div>
      <div className="flex flex-col mb-1">
        <label htmlFor="edit-endpoints-intensity" className={ errs.val && "text-red-600" }>Light intensity ({ intensity })</label>
        <input
          id="edit-endpoints-intensity"
          type="range"
          name="intensity"
          min={0}
          max={MAX_INTENSITY}
          value={ intensity }
          onInput={(e) => {
            setErrs((e) => ({...e, val: ""}));
            setIntensity(parseInt(e.currentTarget.value) || 0);
          }}
        />
      </div>
      <div className="flex flex-col mb-1">
        <label htmlFor="edit-endpoints-watts" className={ errs.watts && "text-red-600" } >Max watts</label>
        <input
          type="number"
          name="width"
          id="edit-endpoints-width"
          min={0}
          value={ watts }
          onInput={ (e) => {
            setErrs((e) => ({...e, watts: ""}));
            setWatts(parseInt(e.currentTarget.value) || 0);
          }}
        />
      </div>
      <div className="flex flex-col mb-1">
        <label htmlFor="edit-endpoints-width" className={ errs.width && "text-red-600" } >Width</label>
        <input
          type="number"
          name="width"
          id="edit-endpoints-width"
          min={0}
          value={ width }
          onInput={ (e) => {
            setErrs((e) => ({...e, width: ""}));
            setWidth(parseInt(e.currentTarget.value) || 0);
          }}
        />
      </div>
      <div className="flex flex-col mb-1">
        <label htmlFor="end-points-height" className={ errs.height && "text-red-600" }>Height</label>
        <input
          type="number"
          name="height"
          id="end-points-height"
          min={0}
          value={height}
          onInput={ (e) => {
            setErrs((e) => ({...e, height: ""}));
            setHeight(parseInt(e.currentTarget.value) || 0);
          }}
        />
      </div>
      <div className="flex flex-col mb-1">
        <label htmlFor="end-points-rotation" className={ errs.rotation && "text-red-600" }>Rotation (deg)</label>
        <input
          type="number"
          name="rotation"
          id="end-points-rotation"
          min={0}
          max={360}
          value={ rotation }
          onInput={ (e) => {
            setErrs((e) => ({...e, rotation: ""}));
            setRotation(parseInt(e.currentTarget.value) || 0);
          }}
        />
      </div>
      <LightPointEditorShowcase height={ height } width={ width } rotation={ rotation } />
      <div className="flex justify-between mt-4">
        <button value="cancel" onClick={ close } className="btn">Cancel</button>
        <button type="submit" className="btn primary" ref={ submitRef }>Save</button>
      </div>
    </form>
  );
}
