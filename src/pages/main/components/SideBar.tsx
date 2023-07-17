import React from 'react';
import { ActionIcon, Navbar, useMantineTheme } from '@mantine/core';
import { SearchBar } from '../../../components/SearchBar';
import FileTreeView from './FileTreeView';
import { IconPlus } from '@tabler/icons-react';
import { type FolderInfo } from '../../../entities/FolderInfo';

interface Props {
  folders: FolderInfo[];
  openWizard: () => void;
}

function SideBar({ folders, openWizard }: Props): JSX.Element {
  const theme = useMantineTheme();

  return (
    <Navbar
      width={{
        base: '30%'
      }}>
      <Navbar.Section>
        <SearchBar>
          <ActionIcon
            size={24}
            radius="xl"
            color={theme.primaryColor}
            variant="filled"
            onClick={openWizard}>
            <IconPlus size="1.1rem" stroke={1.5} />
          </ActionIcon>
        </SearchBar>
      </Navbar.Section>
      <Navbar.Section grow>
        <FileTreeView folders={folders}></FileTreeView>
      </Navbar.Section>
    </Navbar>
  );
}

export default SideBar;
