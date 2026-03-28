import useSWR from "swr";
import { fetchWithToken } from "../_lib/client";
import { Checkout } from "../_types/book";

export const useMyCheckouts = () => {
  const { data, error } = useSWR<{ items: Checkout[] }>(
    "/api/v1/users/me/checkouts",
    (destination) => fetchWithToken(destination),
  );
  return {
    checkouts: data?.items,
    isLoading: !error && !data,
    isError: error,
  };
};

export const useCheckouts = () => {
  const { data, error } = useSWR<{ items: Checkout[] }>(
    "/api/v1/books/checkouts",
    (destination) => fetchWithToken(destination),
  );
  return {
    checkouts: data?.items,
    isLoading: !error && !data,
    isError: error,
  };
};

export const useBookCheckouts = (bookId: string) => {
  const { data, error } = useSWR<{ items: Checkout[] }>(
    `/api/v1/books/${bookId}/checkout-history`,
    (destination) => fetchWithToken(destination),
  );
  return {
    checkouts: data?.items,
    isLoading: !error && !data,
    isError: error,
  };
};


