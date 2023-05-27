export const Button = () => {
  return (
    <button
      style={{
        "background-color": "red",
        color: "white",
      }}
      onClick={() => alert("boop")}
    >
      Boop
    </button>
  );
};
