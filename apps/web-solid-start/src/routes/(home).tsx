import type { VoidComponent } from "solid-js";

import { Button, Input } from "ui";

const Home: VoidComponent = () => {
  return (
    <main class="flex flex-col items-center space-y-4">
      <div>(Header) Web-Solid-Start</div>
      <Button>Button</Button>
      <div>
        <Input value="from ui" />
      </div>
      <Input value="full width" />
    </main>
  );
};

export default Home;
