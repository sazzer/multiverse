import Authentication from "../authentication";
import React from "react";
import landingPicture from "./landing.jpg";

export default () => {
  return (
    <div className="row">
      <div className="col-12 col-md-3 order-sm-3">
        <Authentication />
      </div>
      <div className="col-12 col-md-9">
        <h2>Avaelia</h2>
        <img
          src={landingPicture}
          alt="Avaelia"
          className="img-fluid img-thumbnail rounded shadow"
        />
      </div>
    </div>
  );
};
