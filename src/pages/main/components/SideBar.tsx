import React from 'react';
import { ActionIcon, Navbar } from '@mantine/core';
import { SearchBar } from '../../../components/SearchBar';
import FileTreeView from './FileTreeView';
import { IconPlus } from '@tabler/icons-react';
import { type FolderInfo } from '../../../entities/FolderInfo';

interface Props {
  folders: FolderInfo[];
}

function SideBar({ folders }: Props): JSX.Element {
  return (
    <Navbar
      width={{
        base: 300
      }}>
      <Navbar.Section>
        <SearchBar>
          <ActionIcon variant="default">
            <IconPlus size="1rem" />
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
