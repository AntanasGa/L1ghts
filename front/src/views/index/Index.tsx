import React, { useContext } from "react";
import { AxiosError } from "axios";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import Loader, { Stage } from "../../components/Loader/Loader";
import { ApiContext } from "utils/api";

export default function Index() {
  const [stage, setStage] = useState(Stage.active);
  const [color, setColor] = useState("");
  const [err, setErr] = useState("");
  const navigate = useNavigate();
  const api = useContext(ApiContext);
  useEffect(() => {
    if (!api) {
      return;
    }
    const cancel = api.cancelable();
    api.step.get(cancel.token)
      .then(res => {
        setStage(Stage.complete);
        setColor("shadow-green-400");
        setTimeout(() => {
          switch (res.data.step) {
          case "installed":
            navigate("/login");
            break;
          case "setup":
            navigate("/setup");
            break;
          default:
            setColor("shadow-red-400");
            break;
          }
        }, 1000);
      }).catch((err: AxiosError) => {
        if (err.name === "CanceledError") {
          return;
        }
        setColor("shadow-red-400");
        setErr(err.message);
      });
    return () => {
      cancel.cancel();
    };
  }, [navigate, api]);

          
        
  return (
    <div className="flex items-center place-content-center h-screen">
      <div className="flex flex-col items-center card self-center">
        <Loader stage={ stage } color={ color } />
        {
          err
          && <>{ err }</>
        }
      </div>
    </div>
  );
}
