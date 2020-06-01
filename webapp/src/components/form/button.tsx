import React from "react";
import { useTranslation } from "react-i18next";

export interface ButtonProps {
  label: string;
  loading?: boolean;
  loadingLabel?: string;
  type?: "button" | "submit" | "reset";
  display?: string;
  onClick?: () => void;
}

export const Button: React.FC<ButtonProps> = (props) => {
  const { t } = useTranslation();

  return (
    <>
      <button
        type={props.type || "button"}
        className={`btn btn-${props.display || "primary"}`}
        onClick={props.onClick}
      >
        {props.loading && (
          <>
            <span
              className="spinner-border spinner-border-sm"
              role="status"
              aria-hidden="true"
            ></span>
            &nbsp;
          </>
        )}
        {t(props.label)}
      </button>
      {props.loading && (
        <span
          className="sr-only"
          role="status"
          aria-hidden="false"
          aria-busy="true"
          aria-label={t(props.loadingLabel || "page.spinner.label")}
        ></span>
      )}
    </>
  );
};
