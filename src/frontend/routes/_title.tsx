import { FunctionComponent } from "https://esm.sh/v128/preact@10.19.6/src/index.js";

type Props = {
  title: string;
};

const Title: FunctionComponent<Props> = ({ title }) => {
  return (
    <h1 class="text-4xl font-bold my-2">
      {title}
    </h1>
  );
};

export default Title;
