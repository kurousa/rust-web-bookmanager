export const fetchWithToken = async (destination: string) => {
  return fetcher(destination, {
    headers: {
      "Content-Type": "application/json",
    },
  }).then((res) => res.json());
};

const fetcher = async (destination: string, init: RequestInit) => {
  const res = await fetch(
    `${process.env.API_ROOT_PROTOCOL ?? "http"}://${
      process.env.API_ROOT_URL?.replace(/\/$/g, "") ?? "localhost"
    }:${process.env.API_ROOT_PORT ?? "8080"}/${destination.replace(
      /^\//g,
      "",
    )}`,
    {
      ...init,
      credentials: "include",
    },
  );

  return res;
};

type RequestInfo<T> = {
  destination: string;
  body?: T;
};

const sender = async <T>(
  info: RequestInfo<T>,
  method: "POST" | "PUT" | "DELETE",
) => {
  const headers = {
    "Content-Type": "application/json",
  };
  const basicInit = {
    method: method,
    headers: headers,
  };
  const init = info.body
    ? { ...basicInit, body: JSON.stringify(info.body) }
    : basicInit;
  return fetcher(info.destination, init);
};

export const post = async <T>(info: RequestInfo<T>) => {
  return sender(info, "POST");
};

export const put = async <T>(info: RequestInfo<T>) => {
  return sender(info, "PUT");
};

export const del = async <T>(info: RequestInfo<T>) => {
  return sender(info, "DELETE");
};
