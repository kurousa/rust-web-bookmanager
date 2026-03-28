"use client";

import { Checkout } from "../_types/book";
import { put } from "../_lib/client";
import { Button, useToast } from "@chakra-ui/react";
import { useSWRConfig } from "swr";
import { FC } from "react";

export type ReturnButtonProps = {
  checkout: Checkout;
};

const ReturnButton: FC<ReturnButtonProps> = ({
  checkout,
}: ReturnButtonProps) => {
  const { mutate } = useSWRConfig();
  const toast = useToast();

  const onClickReturningSubmit = async (e: React.SyntheticEvent) => {
    e.preventDefault();
    const res = await put({
      destination: `/api/v1/books/${checkout.book.id}/checkouts/${checkout.id}/returned`,
    });

    if (res.ok) {
      toast({
        title: "蔵書を返却しました",
        description: `「${checkout.book.title}」を返却しました`,
        status: "success",
        duration: 5000,
        isClosable: true,
      });

      mutate("/api/v1/users/me/checkouts");
      mutate("/api/v1/books/checkouts");
    } else {
      toast({
        title: "蔵書を返却できませんでした",
        description: "サーバーからエラー応答が返却されました。",
        status: "error",
        duration: 5000,
        isClosable: true,
      });
    }
  };

  return (
    <Button colorScheme="blue" onClick={onClickReturningSubmit}>
      返却する
    </Button>
  );
};

export default ReturnButton;
