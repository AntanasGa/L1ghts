import React, { useMemo } from "react";

export interface LightPointProps {
  width: number,
  height: number,
  rotation: number,
  intensity?: number,
}

const LightPoint = React.forwardRef<HTMLDivElement, LightPointProps>(({ height, width, rotation, intensity }, ref) => {
  const adjustedIntensity = useMemo(() => (255 * (intensity || 0)) / 1023, [intensity]);
  return (
    <div
      ref={ ref }
      className="bg-white outline outline-black dark:outline-none"
      style={
        {
          height,
          width,
          transform: `rotate(${rotation}deg)`,
          backgroundColor: (intensity !== undefined && `rgb(${adjustedIntensity}, ${adjustedIntensity}, ${adjustedIntensity})`) || undefined,
        }
      }
    ></div>
  );
}
);
LightPoint.displayName = "LightPoint";

export default LightPoint;

// export default function LightPoint({ height, rotation, width, ref }: LightPointProps) {
  
// }
