import { type FolderInfo } from '../../../entities/FolderInfo';
import React from 'react';
import FileTreeView from './FileTreeView';
import VideoFileView from './VideoFileView';
import { Collapse, Group, Text, UnstyledButton } from '@mantine/core';
import { IconArrowDown, IconArrowRight, IconFolder, IconFolderOpen } from '@tabler/icons-react';
import { useDisclosure } from '@mantine/hooks';

interface Props {
  folder: FolderInfo;
}

function renderToggleIcon(folder: FolderInfo, state: boolean): JSX.Element | null {
  if (folder.empty) {
    return null;
  } else if (state) {
    return <IconArrowDown></IconArrowDown>;
  } else {
    return <IconArrowRight></IconArrowRight>;
  }
}

function renderFolderIcon(folder: FolderInfo, state: boolean): JSX.Element | null {
  if (folder.empty || !state) {
    return <IconFolder></IconFolder>;
  } else {
    return <IconFolderOpen></IconFolderOpen>;
  }
}

function FolderInfoView(props: Props): JSX.Element | null {
  const [opened, { toggle }] = useDisclosure(false);
  return (
    <div>
      <UnstyledButton onClick={toggle}>
        <Group>
          {renderToggleIcon(props.folder, false)}
          {renderFolderIcon(props.folder, false)}
          <Text>{props.folder.displayName}</Text>
        </Group>
      </UnstyledButton>
      <Collapse in={opened}>
        <FileTreeView folders={props.folder.folders}></FileTreeView>
        {props.folder.videos.map((video) => (
          <VideoFileView video={video} key={video.path} />
        ))}
      </Collapse>
    </div>
  );
}

export default FolderInfoView;
