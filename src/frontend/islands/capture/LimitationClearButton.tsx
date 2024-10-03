export const LimitationClearButton = () => {
  const handleClick = () => {
    const [rawUrl, params] = String(globalThis.window.location).split("?");
    const res = params.split("&").map((elm) => elm.split("="));
    const map = new Map(res.map((i) => [i[0], i[1]]));
    map.delete("limit");
    let targetParams = "";
    let forIndex = 0;
    map.forEach((val, key) => {
      if (forIndex === 0) {
        targetParams = targetParams.concat(`?`);
      } else {
        targetParams = targetParams.concat(`&`);
      }
      forIndex += 1;
      targetParams = targetParams.concat(`${key}=${val}`);
    });
    globalThis.window.location.href = `${rawUrl}${targetParams}`;
  };

  return (
    <button
      class="bg-slate-200 rounded px-2 py-2 hover:bg-slate-300"
      onClick={handleClick}
    >
      Clear
    </button>
  );
};
