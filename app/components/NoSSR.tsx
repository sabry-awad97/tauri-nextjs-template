import dynamic from "next/dynamic";
import { PropsWithChildren } from "react";

const NoSsr = ({ children }: NonNullable<PropsWithChildren>) => <>{children}</>;
export default dynamic(() => Promise.resolve(NoSsr), { ssr: false });
