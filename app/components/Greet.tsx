"use client";

import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";

export default function Greet() {
  const [greeting, setGreeting] = useState("");

  useEffect(() => {
    invoke<string>("greet", { name: "World" })
      .then((result) => setGreeting(result))
      .catch(console.error);
  }, []);

  return <h1>{greeting}</h1>;
}
