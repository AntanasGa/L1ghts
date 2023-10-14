import { Dispatch, MutableRefObject, SetStateAction, useRef, useState } from "react";
import { TypedUseSelectorHook, useDispatch, useSelector } from "react-redux";
import type { RootState, AppDispatch } from "store";

export const useAppDispatch: () => AppDispatch = useDispatch;
export const useAppSelector: TypedUseSelectorHook<RootState> = useSelector;

export const useRefedState = <T>(v: T): [MutableRefObject<T>, T, Dispatch<SetStateAction<T>>] => {
  const [state, _setState] = useState(v);
  const ref = useRef(state);

  const setState = (v: SetStateAction<T>) => {
    if (v instanceof Function) {
      ref.current = v(ref.current);
    } else {
      ref.current = v;
    }
    _setState(v);
  };
  
  return [ref, state, setState];
};

