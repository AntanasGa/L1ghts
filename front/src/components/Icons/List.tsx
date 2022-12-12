import React from "react";
import { IconProps } from "./types";

export default function List(props: IconProps) {
  const classList = [];
  if (props.classNameArr) {
    classList.push(...props.classNameArr);
  }
  return (
    <svg className={classList.join(" ")} fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg" aria-label="List icon"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M4 6h16M4 10h16M4 14h16M4 18h16"></path></svg>
  );
}
