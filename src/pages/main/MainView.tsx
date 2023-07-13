import React, { useEffect, useState } from 'react';
import { type FolderInfo } from '../../entities/FolderInfo';
import { AppShell, Modal } from '@mantine/core';
import SideBar from './components/SideBar';
import { ScanFile } from '../../service/ScanFile';
import { useDisclosure } from '@mantine/hooks';
import AddFolderWizard from '../addfolder/AddFolderWizard';

function MainView(): JSX.Element {
  const [folders, setFolders] = useState<FolderInfo[]>([]);
  const [wizardOpened, { open, close }] = useDisclosure(false);

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
    <>
      <AppShell padding="md" navbar={<SideBar folders={folders} openWizard={open}></SideBar>}>
        Video View
      </AppShell>
      <Modal opened={wizardOpened} onClose={close} withCloseButton={false} centered>
        <AddFolderWizard></AddFolderWizard>
      </Modal>
    </>
  );
}

export default MainView;
