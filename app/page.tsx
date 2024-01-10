import Greet from "./components/Greet";

export default function Home() {
  return (
    <div className="flex min-h-screen flex-col items-center justify-between p-24">
      <Greet />
    </div>
  );
}
