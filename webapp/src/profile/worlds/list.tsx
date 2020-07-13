import React, { useState } from "react";

import { Link } from "react-router-dom";
import { PageCount } from "../../components/pageCount";
import { Pagination } from "../../components/pagination";

const WorldView: React.FC = () => {
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

export const ListWorldsView: React.FC = () => {
  const [page, setPage] = useState(0);
  return (
    <main aria-label="My Worlds">
      <div className="row">
        <div className="col-12 col-md-9">
          <Pagination current={page} total={7} onClick={setPage} />
        </div>
        <div className="col-12 col-md-3 text-md-right">
          <select className="custom-select" aria-label="Sort worlds by">
            <option>Name</option>
            <option>Created</option>
            <option>Updated</option>
          </select>
        </div>
      </div>
      <div className="list-group list-group-flush" role="list">
        <WorldView />
        <WorldView />
        <WorldView />
        <WorldView />
        <WorldView />
      </div>
      <div>
        <PageCount offset={0} thisPage={5} total={20} />
      </div>
    </main>
  );
};
