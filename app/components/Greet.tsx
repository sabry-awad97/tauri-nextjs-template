"use client";

import rspc from "../trpc/client";

export default function Greet() {
  const { data: name } = rspc.useQuery(["greet", "World"]);
  const { mutate: createUser } = rspc.useMutation("createUser", {
    onSuccess: (data) => {
      console.log("User created!", data);
    },
  });

  return (
    <div>
      <h1>{name}</h1>
      <button
        onClick={() => {
          createUser("Monty Beaumont");
        }}
      >
        Create User!
      </button>
    </div>
  );
}
