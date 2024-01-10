"use client";

import { useEffect, useState } from "react";
import * as commands from "../types/bindings";

export default function Greet() {
  const [greeting, setGreeting] = useState("");

  useEffect(() => {
    commands
      .greet("World")
      .then((result) => setGreeting(result))
      .catch(console.error);
  }, []);

  return <h1>{greeting}</h1>;
}
