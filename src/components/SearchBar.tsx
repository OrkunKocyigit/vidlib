import { Group, TextInput } from '@mantine/core';
import { type ReactNode } from 'react';
import { IconSearch } from '@tabler/icons-react';

interface Props {
  children: ReactNode;
}
export function SearchBar({ children }: Props): JSX.Element {
  return (
    <Group grow>
      <TextInput
        placeholder="Filename"
        icon={<IconSearch size="1rem" stroke={1.5}></IconSearch>}
        rightSection={children}></TextInput>
    </Group>
  );
}
