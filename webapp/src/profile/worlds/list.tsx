import React, { useEffect, useState } from "react";

import { Link } from "react-router-dom";
import { PageCount } from "../../components/pageCount";
import { Pagination } from "../../components/pagination";
import { Spinner } from "../../components/spinner";
import { useUser } from "../../currentUser";

const WorldListItem: React.FC = () => {
  return (
    <Link
      to="/u/sazzer/w/testing"
      className="list-group-item list-group-item-action"
      role="listitem"
    >
      <div className="d-flex w-100 justify-content-between">
        <h5 className="mb-1">List group item heading</h5>
        <small>3 days ago</small>
      </div>
      <p className="mb-1">
        Donec id elit non mi porta gravida at eget metus. Maecenas sed diam eget
        risus varius blandit.
      </p>
    </Link>
  );
};

type WorldsSort = "name" | "created" | "updated";

interface WorldsListProps {
  page: number;
  onPageChange: (page: number) => any;
  sort: WorldsSort;
  onSortChange: (sort: WorldsSort) => any;
}

const WorldsList: React.FC<WorldsListProps> = ({
  page,
  onPageChange,
  sort,
  onSortChange,
}) => {
  return (
    <main aria-label="My Worlds">
      <div className="row">
        <div className="col-12 col-md-9">
          <Pagination current={page} total={7} onClick={onPageChange} />
        </div>
        <div className="col-12 col-md-3 text-md-right">
          <select
            className="custom-select"
            aria-label="Sort worlds by"
            value={sort}
            onChange={(e) => onSortChange(e.target.value as WorldsSort)}
          >
            <option value="name">Name</option>
            <option value="created">Created</option>
            <option value="updated">Updated</option>
          </select>
        </div>
      </div>
      <div className="list-group list-group-flush" role="list">
        <WorldListItem />
        <WorldListItem />
        <WorldListItem />
        <WorldListItem />
        <WorldListItem />
      </div>
      <div>
        <PageCount offset={0} thisPage={5} total={20} />
      </div>
    </main>
  );
};

export const ListWorldsView: React.FC = () => {
  const { userLink } = useUser();
  const [page, setPage] = useState(0);
  const [sort, setSort] = useState<WorldsSort>("name");

  useEffect(() => {
    console.log("Searching worlds", userLink, page, sort);
  }, [userLink, page, sort]);

  return userLink == null ? (
    <Spinner />
  ) : (
    <WorldsList
      page={page}
      onPageChange={setPage}
      sort={sort}
      onSortChange={setSort}
    />
  );
};
