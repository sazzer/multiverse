import React, { useEffect, useState } from "react";

import { Link } from "react-router-dom";
import { PageCount } from "../../components/pageCount";
import { Pagination } from "../../components/pagination";
import { Spinner } from "../../components/spinner";
import { useUser } from "../../currentUser";
import type { WorldsSortField, PagedResults } from "../../api/worlds/search";
import type { World } from "../../api/worlds";
import { searchWorlds } from "../../api/worlds";

/** The number of items in a single page */
const PAGE_SIZE = 6;

interface WorldListItemProps {
  world: World;
}

const WorldListItem: React.FC<WorldListItemProps> = ({ world }) => {
  return (
    <Link
      to={`/u/${"sazzer"}/w/${world.slug}`}
      className="list-group-item list-group-item-action"
      role="listitem"
    >
      <div className="d-flex w-100 justify-content-between">
        <h5 className="mb-1">{world.name}</h5>
      </div>
      <p className="mb-1">{world.description}</p>
    </Link>
  );
};

interface WorldsListProps {
  worlds: PagedResults<World>;
  page: number;
  onPageChange: (page: number) => any;
  sort: WorldsSortField;
  onSortChange: (sort: WorldsSortField) => any;
}

const WorldsList: React.FC<WorldsListProps> = ({
  worlds,
  page,
  onPageChange,
  sort,
  onSortChange,
}) => {
  const worldEntries = worlds.entries.map((world, index) => (
    <WorldListItem key={index} world={world} />
  ));

  return (
    <main aria-label="My Worlds">
      <div className="row">
        <div className="col-12 col-md-9">
          <Pagination
            current={page}
            total={worlds.pagination.total / worlds.pagination.count}
            onClick={onPageChange}
          />
        </div>
        <div className="col-12 col-md-3 text-md-right">
          <select
            className="custom-select"
            aria-label="Sort worlds by"
            value={sort}
            onChange={(e) => onSortChange(e.target.value as WorldsSortField)}
          >
            <option value="name">Name</option>
            <option value="created">Created</option>
            <option value="updated">Updated</option>
          </select>
        </div>
      </div>
      <div className="list-group list-group-flush" role="list">
        {worldEntries}
      </div>
      <div>
        <div className="row align-items-center">
          <div className="col-12 col-md-9">
            <PageCount
              offset={worlds.pagination.offset}
              thisPage={worlds.entries.length}
              total={worlds.pagination.total}
            />
          </div>
          <div className="col-12 col-md-3 text-md-right">
            <Link to="/profile/worlds/new" className="btn btn-primary">
              New World
            </Link>
          </div>
        </div>
      </div>
    </main>
  );
};

export const ListWorldsView: React.FC = () => {
  const { userLink } = useUser();
  const [page, setPage] = useState(0);
  const [sort, setSort] = useState<WorldsSortField>("name");
  const [worlds, setWorlds] = useState<PagedResults<World> | null>(null);

  useEffect(() => {
    setWorlds(null);
    searchWorlds(
      {
        owner: userLink,
      },
      {
        offset: page * PAGE_SIZE,
        count: PAGE_SIZE,
      },
      [
        {
          field: sort,
        },
      ]
    ).then(setWorlds);
  }, [userLink, page, sort]);

  return userLink == null || worlds == null ? (
    <Spinner />
  ) : (
    <WorldsList
      worlds={worlds}
      page={page}
      onPageChange={setPage}
      sort={sort}
      onSortChange={setSort}
    />
  );
};
