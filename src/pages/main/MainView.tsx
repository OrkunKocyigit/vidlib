import React, { useEffect, useState } from 'react';
import { type FolderInfo } from '../../entities/FolderInfo';
import { AppShell } from '@mantine/core';
import SideBar from './components/SideBar';
import { ScanFile } from '../../service/ScanFile';

function MainView(): JSX.Element {
  const [folders, setFolders] = useState<FolderInfo[]>([]);

  useEffect(() => {
    ScanFile('C:\\Projects\\vidlib\\test')
      .then((response) => {
        const { response: folderInfo } = response;
        if (folderInfo != null) {
          setFolders([folderInfo]);
        }
      })
      .catch((reason): void => {
        console.error(reason);
      });
  }, []);

  return (
    <AppShell padding="md" navbar={<SideBar folders={folders}></SideBar>}>
      Video View
    </AppShell>
  );
}

export default MainView;
