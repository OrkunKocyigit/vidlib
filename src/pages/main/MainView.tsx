import React, { useEffect, useState } from 'react';
import { type FolderInfo } from '../../entities/FolderInfo';
import { AppShell, Modal } from '@mantine/core';
import SideBar from './components/SideBar';
import { useDisclosure } from '@mantine/hooks';
import AddFolderWizard from '../addfolder/AddFolderWizard';
import { GetFolders } from '../../service/GetFolders';
import VideoView from '../video/VideoView';
import { type VideoFile } from '../../entities/VideoFile';
import { type IVideoContext, VideoContext } from './entities/VideoContext';

function MainView(): JSX.Element {
  const [folders, setFolders] = useState<FolderInfo[]>([]);
  const [wizardOpened, { open, close }] = useDisclosure(false);
  const [video, setVideo] = useState<VideoFile | undefined>();
  const videoContext: IVideoContext = { video, setVideo };

  useEffect(() => {
    GetFolders()
      .then((response) => {
        const { response: folderInfos } = response;
        if (folderInfos != null) {
          setFolders(folderInfos);
        }
      })
      .catch((reason): void => {
        console.error(reason);
      });
  }, []);

  function onFolderAdd(folderInfo: FolderInfo): void {
    setFolders([...folders, folderInfo]);
  }

  return (
    <>
      <AppShell
        padding="md"
        navbar={
          <VideoContext.Provider value={videoContext}>
            <SideBar folders={folders} openWizard={open}></SideBar>
          </VideoContext.Provider>
        }>
        <VideoView video={video}></VideoView>
      </AppShell>
      <Modal opened={wizardOpened} onClose={close} withCloseButton={false} centered>
        <AddFolderWizard onFolderAdd={onFolderAdd}></AddFolderWizard>
      </Modal>
    </>
  );
}

export default MainView;
