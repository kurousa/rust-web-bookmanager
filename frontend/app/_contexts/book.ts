import useSWR from "swr";
import { Book, PaginatedList } from "../_types/book";
import { fetchWithToken } from "../_lib/client";

type BooksQuery = {
  limit: number;
  offset: number;
};

export const useBooks = (query: BooksQuery) => {
  const { data, error } = useSWR<PaginatedList<Book>>(
    `/api/v1/books?limit=${query.limit}&offset=${query.offset}`,
    (destination) => fetchWithToken(destination),
  );
  return {
    books: data,
    isLoading: !error && !data,
    isError: error,
  };
};

export const useBook = (id: string) => {
  const { data, error } = useSWR<Book>(`/api/v1/books/${id}`, (destination) =>
    fetchWithToken(destination),
  );
  return {
    book: data,
    isLoading: !error && !data,
    isError: error,
  };
};
