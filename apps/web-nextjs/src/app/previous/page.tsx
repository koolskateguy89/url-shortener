import { Button, Input } from "ui";

export default function Page() {
  return (
    <main className="flex flex-col items-center space-y-4">
      <div>(Header) Web-Nextjs</div>
      <Button>Button</Button>
      <button className="bg-destructive text-destructive-foreground hover:bg-destructive/90 bg-red-500">
        aaa
      </button>
      <div>
        <Input defaultValue="from ui" />
      </div>
      <Input defaultValue="full width" />
    </main>
  );
}
