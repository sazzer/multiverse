import { Link } from "react-router-dom";
import { Pagination } from "../../components/pagination";
import React from "react";

const WorldView: React.FC = () => {
  return (
    <Link
      to="/u/sazzer/w/testing"
      className="list-group-item list-group-item-action"
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

const pageTests: any[] = [];
const total = 15;
for (let p = 0; p < total; ++p) {
  pageTests.push(
    <Pagination
      key={p}
      current={p}
      total={total}
      onClick={(p) => console.log(p)}
    />
  );
}

export const ListWorldsView: React.FC = () => {
  return (
    <div>
      {pageTests}
      <div className="list-group list-group-flush">
        <WorldView />
        <WorldView />
        <WorldView />
        <WorldView />
        <WorldView />
      </div>
    </div>
  );
};
