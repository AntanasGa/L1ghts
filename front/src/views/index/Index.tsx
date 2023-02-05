import React, { useContext, useRef } from "react";
import { AxiosError, Canceler } from "axios";
import { useEffect, useState } from "react";
import { useNavigate } from "react-router-dom";
import Loader, { Stage } from "../../components/Loader/Loader";
import { ApiContext } from "utils/api";

export default function Index() {
  const navigate = useNavigate();
  const api = useContext(ApiContext);

  const didMount = useRef(false);

  const [stage, setStage] = useState(Stage.active);
  const [color, setColor] = useState("");
  const [message, setMessage] = useState("Validating access");
  const [loaderClasses, setLoaderClasses] = useState(["flex flex items-center card self-center ease-in duration-150", "mb-[-50%]"]);

  const checkApi = () => {
    if (!api || !didMount.current) {
      return;
    }
    const cancel = api.cancelable();
    api.step.get(cancel.token)
      .then(res => {
        setStage(Stage.complete);
        setColor("shadow-green-400");
        setTimeout(() => {
          setLoaderClasses([loaderClasses[0], "mb-[-50%]"]);
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
          }, 500);
        }, 250);
      }).catch((err: AxiosError) => {
        if (err.name === "CanceledError") {
          return;
        }
        setColor("shadow-red-400");
        setMessage(err.message);
      });
    return cancel.cancel;
  };
  
  useEffect(() => {
    let cancelFn: Canceler | undefined;
    if (!didMount.current) {
      didMount.current = true;
      setLoaderClasses((v) => [v[0], "mb-3"]);
      cancelFn = checkApi();
    }
    // everyTick
    return () => {
      didMount.current = false;
      cancelFn?.();
    };
  // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [api]);

          
        
  return (
    <div className="fixed bottom-0 left-0 right-0 overflow-hidden flex items-center place-content-center">
      <div className={ loaderClasses.join(" ") }>
        <Loader stage={ stage } color={ color } size={ 16 } />
        {
          message
          && <>{ message }</>
        }
      </div>
    </div>
  );
}
