import { Button, Input } from "ui";

export default function Page() {
  return (
    <main className="flex flex-col items-center space-y-4">
      <div>(Header) Web-Nextjs</div>
      <Button>Button</Button>
      <div>
        <Input defaultValue="from ui" />
      </div>
      <Input defaultValue="full width" />
    </main>
  );
}
