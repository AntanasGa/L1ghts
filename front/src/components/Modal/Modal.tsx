import React, { MouseEvent, useEffect, useRef } from "react";
import ReactDOM from "react-dom";

export interface InitialModalProps extends React.PropsWithChildren {
  portalKey: string,
  active?: boolean,
  label?: string,
}

type ModalProps = InitialModalProps
& (
  {
    programClose: true,
    onClose?: (e: React.SyntheticEvent<HTMLDialogElement, Event>) => void,
  }
  |
  {
    programClose?: false,
    onClose: (e: React.SyntheticEvent<HTMLDialogElement, Event>) => void,
  }
)

export default function Modal(props: ModalProps) {
  const dialogRef = useRef<HTMLDialogElement|null>(null);
  const dialogActive = useRef(false);
  const modals = document.getElementById("modals");

  useEffect(() => {
    const effectDialog = dialogRef.current;
    if (effectDialog) {
      if (props.active && !dialogActive.current) {
        dialogActive.current = true;
        effectDialog.showModal();
      }
      if (!props.active && dialogActive.current) {
        dialogActive.current = false;
        effectDialog.close();
      }
    }
    return () => {
      dialogActive.current = false;
      effectDialog?.close();
    };
  }, [props.active]);
  if (!props.children || !modals) {
    return <></>;
  }

  function onBackdropClick(e: MouseEvent<HTMLDialogElement>) {
    if (props.programClose) {
      return;
    }
    const rect = e.currentTarget.getBoundingClientRect();
    const clickInModal = rect.top <= e.clientY && rect.bottom >= e.clientY
      && rect.left <= e.clientX && rect.right >= e.clientX;
    if (!clickInModal) {
      e.currentTarget.close();
    }
  }

  return ReactDOM.createPortal(
    (
      // dialog already has a keyboard handler, this is not a useful eslint thing
      // eslint-disable-next-line jsx-a11y/no-noninteractive-element-interactions, jsx-a11y/click-events-have-key-events
      <dialog
        ref={ dialogRef }
        className={ "card backdrop:backdrop-blur-md" }
        onMouseDown={ onBackdropClick }
        onClose={ props.onClose }
        aria-label={ props.label }
      >
        { props.active && props.children }
      </dialog>
    ),
    modals,
    props.portalKey,
  );
}