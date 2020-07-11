import React from "react";
import { useTranslation } from "react-i18next";

export interface PaginationProps {
  current: number;
  total: number;
  maxPageButtons?: number;
  onClick: (page: number) => any;
}

export const Pagination: React.FC<PaginationProps> = ({
  current,
  total,
  maxPageButtons,
  onClick,
}) => {
  const { t } = useTranslation();

  if (total <= 1 && current === 0) {
    // If we only have a single page, don't bother rendering anything
    return <></>;
  }

  const realMaxPageButtons = maxPageButtons || 5;

  const hasPrevious = current > 0;
  const hasNext = current + 1 < total;

  const pageButtons: number[] = [];
  for (let p = 0; p < total; ++p) {
    pageButtons.push(p);
  }
  while (pageButtons.length > realMaxPageButtons) {
    if (current - Math.floor(realMaxPageButtons / 2) > pageButtons[0]) {
      pageButtons.shift();
    } else {
      pageButtons.pop();
    }
  }

  const buttonElements = pageButtons.map((p) => {
    const isCurrent = p === current;
    return (
      <li
        className={`page-item ${isCurrent ? "active" : ""}`}
        aria-current={isCurrent}
        key={p}
      >
        <button
          className="page-link"
          onClick={() => onClick(p)}
          aria-label={t("page.pagination.page", { page: p + 1 })}
        >
          {p + 1}
        </button>
      </li>
    );
  });

  return (
    <nav aria-label={t("page.pagination.label")}>
      <ul className="pagination">
        <li className={`page-item ${hasPrevious ? "" : "disabled"}`}>
          <button
            className="page-link"
            aria-label={t("page.pagination.previous")}
            onClick={() => onClick(current - 1)}
          >
            <span aria-hidden="true">&laquo;</span>
          </button>
        </li>
        {buttonElements}
        <li className={`page-item ${hasNext ? "" : "disabled"}`}>
          <button
            className="page-link"
            aria-label={t("page.pagination.next")}
            onClick={() => onClick(current + 1)}
          >
            <span aria-hidden="true">&raquo;</span>
          </button>
        </li>
      </ul>
    </nav>
  );
};
