import useSWR from "swr";
import { fetchWithToken, post } from "../_lib/client";
import { User, Users } from "../_types/user";
import { useRouter } from "next/navigation";

export const useLogout = () => {
  const router = useRouter();

  const logout = async () => {
    await post({ destination: "/auth/logout" });
    router.push("/login");
  };

  return { logout };
};

export const useCurrentUser = () => {
  const { data, error } = useSWR<User>("/api/v1/users/me", (destination) =>
    fetchWithToken(destination),
  );
  return {
    currentUser: data,
    isLoading: !error && !data,
    isError: error,
  };
};

export const useUsers = () => {
  const { data, error } = useSWR<Users>("/api/v1/users", (destination) =>
    fetchWithToken(destination),
  );
  return {
    users: data,
    isLoading: !error && !data,
    isError: error,
  };
};
