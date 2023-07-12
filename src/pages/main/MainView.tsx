import React from 'react';
import { type FolderInfo } from '../../entities/FolderInfo';
import { AppShell } from '@mantine/core';
import SideBar from './components/SideBar';
import { ScanFile } from '../../service/ScanFile';

interface Props {}

interface State {
  folders: FolderInfo[];
}

class MainView extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = {
      folders: []
    };
  }

  componentDidMount(): void {
    ScanFile('C:\\Projects\\vidlib\\test')
      .then((response) => {
        const { response: folderInfo } = response;
        if (folderInfo != null) {
          this.setState((prevState) => ({
            folders: [folderInfo]
          }));
        }
      })
      .catch((reason): void => {
        console.error(reason);
      });
  }

  render(): JSX.Element {
    return (
      <AppShell padding="md" navbar={<SideBar folders={this.state.folders}></SideBar>}>
        Video View
      </AppShell>
    );
  }
}

export default MainView;
