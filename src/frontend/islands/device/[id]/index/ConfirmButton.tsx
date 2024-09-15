export const ConfirmButton = (
  props: { text: string; confirmText: string; url: string },
) => {
  const handleClick = () => {
    const res = globalThis.window.confirm("really delete?");
    if (res) {
      globalThis.window.location.href = `${props.url}`;
    }
  };

  return (
    <button
      class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded mb-4"
      onClick={handleClick}
    >
      {props.text}
    </button>
  );
};
