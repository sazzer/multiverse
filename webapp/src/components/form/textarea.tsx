import "./input.css";

import { FieldError } from "react-hook-form";
import React from "react";
import { useTranslation } from "react-i18next";

export interface TextareaProps {
  id: string;
  name?: string;
  i18n: string;
  error?: FieldError;
  required?: boolean;
  autoFocus?: boolean;
  readOnly?: boolean;
  rows?: number;
  textareaProps?: { [key: string]: any };
  describedBy?: string;
}

export const Textarea: React.FC<TextareaProps> = (props) => {
  const { t } = useTranslation();

  const labelClass = props.required ? "multiverse-required" : "";
  return (
    <>
      <label htmlFor={props.id}>
        {t(`${props.i18n}.label`)}
        <span className={labelClass} aria-hidden="true"></span>
      </label>
      <textarea
        rows={props.rows || 5}
        className={`form-control ${props.error ? "is-invalid" : ""}`}
        id={props.id}
        name={props.name || props.id}
        required={props.required}
        aria-required={!!props.required}
        aria-invalid={props.error ? "true" : "false"}
        aria-describedby={`${props.id}-error ${props.describedBy || ""}`}
        autoFocus={props.autoFocus}
        readOnly={props.readOnly}
        {...props.textareaProps}
      />
      {props.error && (
        <div className="invalid-feedback" id={`${props.id}-error`} role="alert">
          {t(`${props.i18n}.errors.${props.error.type}`)}
        </div>
      )}
    </>
  );
};
