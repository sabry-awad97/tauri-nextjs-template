"use client";

import { createClient } from "@rspc/client";
import { TauriTransport } from "@rspc/tauri";
import { QueryClient } from "@tanstack/react-query";
import { PropsWithChildren, useState } from "react";
import { Procedures } from "./trpc/bindings";
import rspc from "./trpc/client";

const Providers = ({ children }: NonNullable<PropsWithChildren>) => {
  const [queryClient] = useState(() => new QueryClient());
  const [trpcClient] = useState(() =>
    createClient<Procedures>({
      transport: new TauriTransport(),
    })
  );

  return (
    <rspc.Provider client={trpcClient} queryClient={queryClient}>
      <>{children}</>
    </rspc.Provider>
  );
};

export default Providers;
