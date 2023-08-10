import { Group, TextInput } from '@mantine/core';
import { type ReactNode } from 'react';
import { IconSearch } from '@tabler/icons-react';
import { useTranslation } from 'react-i18next';

interface Props {
  children: ReactNode;
}
export function SearchBar({ children }: Props): JSX.Element {
  const { t } = useTranslation();
  return (
    <Group grow>
      <TextInput
        placeholder={t('searchbar.placeholder')}
        icon={<IconSearch size="1rem" stroke={1.5}></IconSearch>}
        rightSection={children}></TextInput>
    </Group>
  );
}
