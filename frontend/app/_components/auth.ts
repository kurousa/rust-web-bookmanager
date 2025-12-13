import { redirect } from "next/navigation";
import { FC } from "react";
import useLocalStorageState from "use-local-storage-state";

const AuthProvider: FC<{ children: React.ReactNode }> = ({ children }) => {
  const [accessToken] = useLocalStorageState(ACCESS_TOKEN_KEY);

  if (accessToken === undefined) {
    redirect("/login");
  }

  return children;
};

export default AuthProvider;

const accessTokenKey = process.env.NEXT_PUBLIC_ACCESS_TOKEN_KEY;

if (!accessTokenKey) {
  throw new Error("NEXT_PUBLIC_ACCESS_TOKEN_KEY is not defined");
}

export const ACCESS_TOKEN_KEY = accessTokenKey;
