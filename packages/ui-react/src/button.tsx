"use client";

import * as React from "react";

export const Button = () => {
  return (
    <button
      style={{
        backgroundColor: "red",
        color: "white",
      }}
      onClick={() => alert("boop")}
    >
      Boop
    </button>
  );
};
