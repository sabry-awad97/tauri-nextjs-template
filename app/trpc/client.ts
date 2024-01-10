import { createReactQueryHooks } from "@rspc/react";
import type { Procedures } from "./bindings";

const rspc = createReactQueryHooks<Procedures>();
export default rspc;
