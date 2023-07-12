import React from 'react';
import { type FolderInfo } from '../../../entities/FolderInfo';
import { ActionIcon, Navbar } from '@mantine/core';
import { SearchBar } from '../../../components/SearchBar';
import FileTreeView from './FileTreeView';
import { IconPlus } from '@tabler/icons-react';

interface Props {
  folders: FolderInfo[];
}

interface State {}
class SideBar extends React.Component<Props, State> {
  render(): JSX.Element {
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
          <FileTreeView folders={this.props.folders}></FileTreeView>
        </Navbar.Section>
      </Navbar>
    );
  }
}

export default SideBar;
