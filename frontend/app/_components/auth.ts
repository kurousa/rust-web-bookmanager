import { redirect } from "next/navigation";
import { FC } from "react";
import { useCurrentUser } from "../_contexts/user";

const AuthProvider: FC<{ children: React.ReactNode }> = ({ children }) => {
  const { currentUser, isLoading, isError } = useCurrentUser();

  if (isLoading) {
    return null;
  }

  if (isError || !currentUser) {
    redirect("/login");
  }

  return children;
};

export default AuthProvider;
