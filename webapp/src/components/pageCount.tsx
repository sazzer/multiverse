import React from "react";
import { useTranslation } from "react-i18next";

export interface PageCountProps {
  offset: number;
  total: number;
  thisPage: number;
}

export const PageCount: React.FC<PageCountProps> = (props) => {
  const { t } = useTranslation();

  const first = props.offset + 1;
  const last = props.offset + props.thisPage;

  if (props.total > 0) {
    return <>{t("page.pageCount", { first, last, total: props.total })}</>;
  } else {
    return <></>;
  }
};
