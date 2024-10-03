export const LimitationDataForm = () => {
  const handleClick = () => {
    const limitStartDate =
      globalThis.document.getElementById("limit-start-date").value;

    const limitEndDate =
      globalThis.document.getElementById("limit-end-date").value;
    if (limitStartDate === "" || limitEndDate === "") {
    } else {
      const [rawUrl, params] = String(globalThis.window.location).split("?");
      const res = params.split("&").map((elm) => elm.split("="));
      const map = new Map(res.map((i) => [i[0], i[1]]));
      map.set("start_date", limitStartDate);
      map.set("end_date", limitEndDate);
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
    }
  };

  const [_initRawUrl, initParams] = String(globalThis.window.location).split(
    "?",
  );
  let start_date: string | undefined = "";
  let end_date: string | undefined = "";
  if (initParams) {
    const initRes = initParams.split("&").map((elm) => elm.split("="));
    const map = new Map(initRes.map((i) => [i[0], i[1]]));
    start_date = map.get("start_date");
    end_date = map.get("end_date");
  }

  return (
    <div>
      <input
        class="bg-slate-200 rounded px-2 py-2 hover:bg-slate-300"
        type="datetime-local"
        value={start_date}
        id="limit-start-date"
      />
      ~
      <input
        class="bg-slate-200 rounded px-2 py-2 hover:bg-slate-300"
        type="datetime-local"
        value={end_date}
        id="limit-end-date"
      />
      <button
        onClick={handleClick}
        class="bg-slate-200 rounded px-2 py-2 hover:bg-slate-300"
      >
        Set
      </button>
    </div>
  );
};
