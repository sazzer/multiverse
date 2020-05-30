import { FieldError } from "react-hook-form";
import React from "react";
import { useTranslation } from "react-i18next";

export interface InputProps {
  id: string;
  name?: string;
  i18n: string;
  type?: string;
  error?: FieldError;
  required?: boolean;
  autoFocus?: boolean;
  readOnly?: boolean;
  inputProps?: { [key: string]: any };
}

export const Input: React.FC<InputProps> = (props) => {
  const { t } = useTranslation();

  return (
    <div className="form-group">
      <label htmlFor={props.id}>{t(`${props.i18n}.label`)}</label>
      <input
        type={props.type || "text"}
        className={`form-control ${props.error && "is-invalid"}`}
        id={props.id}
        name={props.name || props.id}
        required={props.required}
        autoFocus={props.autoFocus}
        readOnly={props.readOnly}
        {...props.inputProps}
      />
      {props.error && (
        <div className="invalid-feedback">
          {t(`${props.i18n}.errors.${props.error.type}`)}
        </div>
      )}
    </div>
  );
};
