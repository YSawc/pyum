import { JSX } from "preact/jsx-runtime";

const handleClick = () => {
  console.log("clicked");
};

export const Button = (
  props: { event: JSX.MouseEventHandler; text: string },
) => {
  return (
    <button
      class="bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded mb-4"
      onClick={() => props.event()}
    >
      {props.text}
    </button>
  );
};
