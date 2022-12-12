import React, { useEffect, useRef, useState } from "react";
import LightPoint from "components/LightPoint/LightPoint";

export interface LightPointEditorShowcaseProps {
  width: number,
  height: number,
  rotation: number,
}

export default function LightPointEditorShowcase({height, rotation, width}: LightPointEditorShowcaseProps) {
  // because the platform is square we only need one variable
  const [ wrapperWidth, setWrapperWidth ] = useState(-1);
  const [scale, setScale] = useState(1);

  const isMounted = useRef(false);

  const winWidth = useRef(window.innerWidth);
  const lastUpdate = useRef<NodeJS.Timeout|null>(null);

  const wrapper = useRef<HTMLDivElement|null>(null);
  const lightSource = useRef<HTMLDivElement|null>(null);

  useEffect(() => {
    // mount
    if (!isMounted.current) {
      isMounted.current = true;
      window.addEventListener("resize", onWindowResize);
      setWrapperWidth(wrapper.current?.offsetWidth || -1);
    }

    // unmount
    return () => {
      isMounted.current = false;
      window.removeEventListener("resize", onWindowResize);
    };
  }, []);

  useEffect(() => {
    if (wrapperWidth === -1) {
      setScale(1);
    }
    setWrapperWidth(wrapper.current?.offsetWidth || -1);
  }, [wrapperWidth]);

  // initiate scale update
  useEffect(() => {
    setScale(1);
  }, [rotation, height, width]);

  useEffect(() => {
    if (!lightSource.current || wrapperWidth === -1 || scale !== 1) {
      return;
    }
    
    const { width: w, height: h } = lightSource.current.getBoundingClientRect();
    
    const scaleBy = w > h ? w : h;
    
    setScale(wrapperWidth / scaleBy);
    
  }, [scale, wrapperWidth]);

  function onWindowResize(_: UIEvent) {
    if (lastUpdate.current != null) {
      clearTimeout(lastUpdate.current);
    }
    if (winWidth.current === window.innerWidth) {
      return;
    }
    lastUpdate.current = setTimeout(() => {
      if (lastUpdate.current !== null) {
        clearTimeout(lastUpdate.current);
        lastUpdate.current = null;
      }
      if (winWidth.current === window.innerWidth) {
        return;
      }
      winWidth.current = window.innerHeight;

      setWrapperWidth(-1);
    }, 250);
  }
  
  return (
    <div>
      <div
        ref={ wrapper }
        style={{
          // aspect ratio doesn't work on ios devices can't have aspect-square
          width: wrapperWidth === -1 ? undefined : wrapperWidth,
          height: wrapperWidth === -1 ? undefined : wrapperWidth,
        }}
        className="flex flex-col items-center justify-center overflow-hidden"
      >
        { wrapperWidth !== -1 &&
        <div
          style={{
            transform: `scale(${scale})`,
          }}
        >

          <LightPoint ref={ lightSource } height={ height } width={ width } rotation={ rotation } />
        </div>
        }
      </div>
    </div>
  );
}
