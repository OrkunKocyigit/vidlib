import { Box, Group, rem, TextInput } from '@mantine/core';
import { type ReactNode } from 'react';
import { IconSearch } from '@tabler/icons-react';

interface Props {
  children: ReactNode;
}
export function SearchBar({ children }: Props): JSX.Element {
  return (
    <Box
      sx={(theme) => ({
        paddingTop: theme.spacing.xs,
        paddingLeft: theme.spacing.xs,
        paddingRight: theme.spacing.xs,
        paddingBottom: theme.spacing.md,
        borderBottom: `${rem(1)} solid ${
          theme.colorScheme === 'dark' ? theme.colors.dark[4] : theme.colors.gray[2]
        }`
      })}>
      <Group grow>
        <TextInput
          placeholder="Filename"
          icon={<IconSearch size="1rem" stroke={1.5}></IconSearch>}
          rightSection={children}></TextInput>
      </Group>
    </Box>
  );
}
