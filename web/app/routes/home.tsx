import { Welcome } from "~/welcome/welcome";
import type { Route } from "./+types/home";
import { PackageList } from "~/package-list/package-list";

export function meta({}: Route.MetaArgs) {
  return [
    { title: "Aether Pub" },
    { name: "description", content: "Welcome to Aether Pub!" },
  ];
}

export default function Home() {
  return <PackageList />;
  return <Welcome />;
}
