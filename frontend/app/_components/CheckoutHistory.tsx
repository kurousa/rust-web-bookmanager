import {
  Table,
  Thead,
  Tbody,
  Tr,
  Th,
  Td,
  TableContainer,
} from "@chakra-ui/react";
import { useBookCheckouts } from "../_contexts/checkout";
import { useUsers } from "../_contexts/user";
import { User } from "../_types/user";
import { FC, useMemo } from "react";

type CheckoutHistoryProps = {
  bookId: string;
};

const CheckoutHistory: FC<CheckoutHistoryProps> = ({
  bookId,
}: CheckoutHistoryProps) => {
  const { checkouts } = useBookCheckouts(bookId);
  const { users } = useUsers();
  const userItems = users?.items;

  const userMap = useMemo(() => {
    const map = new Map<string, User>();
    userItems?.forEach((user) => {
      map.set(user.id, user);
    });
    return map;
  }, [userItems]);

  return (
    <TableContainer>
      <Table variant="simple">
        <Thead>
          <Tr>
            <Th>貸出日</Th>
            <Th>返却日</Th>
            <Th>貸出者</Th>
          </Tr>
        </Thead>
        <Tbody>
          {checkouts?.map((co) => (
            <Tr key={co.id}>
              <Td>{co.checkedOutAt}</Td>
              <Td>{co.returnedAt ?? "-"}</Td>
              <Td>{userMap.get(co.checkedOutBy)?.name}</Td>
            </Tr>
          ))}
        </Tbody>
      </Table>
    </TableContainer>
  );
};

export default CheckoutHistory;
