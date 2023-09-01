import React from 'react';
import { ActionIcon, Navbar, ScrollArea, useMantineTheme } from '@mantine/core';
import { SearchBar } from '../../../components/SearchBar';
import FileTreeView from './FileTreeView';
import { IconPlus } from '@tabler/icons-react';
import { type FolderInfo } from '../../../entities/FolderInfo';
import useStyles from './SideBar.styles';

interface Props {
  folders: FolderInfo[];
  openWizard: () => void;
}

function SideBar({ folders, openWizard }: Props): JSX.Element {
  const theme = useMantineTheme();
  const { classes } = useStyles();

  return (
    <Navbar
      width={{
        base: '30%'
      }}
      className={classes.navbar}
    >
      <Navbar.Section className={classes.header}>
        <SearchBar>
          <ActionIcon
            size={24}
            radius="xl"
            color={theme.primaryColor}
            variant="filled"
            onClick={openWizard}
          >
            <IconPlus size="1.1rem" stroke={1.5} />
          </ActionIcon>
        </SearchBar>
      </Navbar.Section>
      <Navbar.Section grow className={classes.files} component={ScrollArea}>
        <FileTreeView folders={folders} showDelete={true}></FileTreeView>
      </Navbar.Section>
    </Navbar>
  );
}

export default SideBar;
