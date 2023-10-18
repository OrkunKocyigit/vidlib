import React from 'react';
import { ActionIcon, AppShell, ScrollArea, useMantineTheme } from '@mantine/core';
import { SearchBar } from '../../../components/SearchBar';
import FileTreeView from './FileTreeView';
import { IconPlus } from '@tabler/icons-react';
import { type FolderInfo } from '../../../entities/FolderInfo';
import classes from './SideBar.module.pcss';

interface Props {
  folders: FolderInfo[];
  openWizard: () => void;
}

function SideBar({ folders, openWizard }: Props): JSX.Element {
  const theme = useMantineTheme();

  return (
    <AppShell.Navbar className={classes.navbar}>
      <div className={classes.header}>
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
      </div>
      <ScrollArea className={classes.files}>
        <FileTreeView folders={folders} showDelete={true}></FileTreeView>
      </ScrollArea>
    </AppShell.Navbar>
  );
}

export default SideBar;
