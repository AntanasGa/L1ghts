import React from "react";

export enum Stage {
    inactive = 0,
    active = 1,
    complete = 2,
}

export interface LoaderProps {
    color?: string,
    stage?: Stage,
    // tailwind preloaded variables
    size?: 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 10 | 12 | 14 | 16,
}

export default function Loader(props: LoaderProps) {
  const classList = ["circle"];
  const loaderClassList = ["loader"];
  if (props.stage) {
    classList.push(props.stage === Stage.active ? "active" : "complete");
  }
  if (props.color && props.color.trim()) {
    classList.push(props.color);
  } else {
    classList.push(...["dark:shadow-white", "shadow-black"]);
  }
  if (props.size && props.size % 1 === 0) {
    // tailwind autoloading does not pick up variables, this should be enough
    // w-1 w-2 w-3 w-4 w-5 w-6 w-7 w-8 w-10 w-12 w-14 w-16
    // h-1 h-2 h-3 h-4 h-5 h-6 h-7 h-8 h-10 h-12 h-14 h-16
    loaderClassList.push(...[`w-${props.size}`, `h-${props.size}`, `md:w-${props.size}`, `md:h-${props.size}`,]);
  }
  return (
    <div className={loaderClassList.join(" ")} data-testid="loader-element">
      <div className={classList.join(" ")} data-testid="loader-circle-element"></div>
    </div>
  );
}