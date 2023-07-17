import { type FolderInfo } from '../../../entities/FolderInfo';
import React from 'react';
import FileTreeView from './FileTreeView';
import VideoFileView from './VideoFileView';
import { Collapse, Group, Text, UnstyledButton } from '@mantine/core';
import { IconArrowDown, IconArrowRight, IconFolder, IconFolderOpen } from '@tabler/icons-react';
import { useDisclosure } from '@mantine/hooks';
import styled from 'styled-components';

interface Props {
  folder: FolderInfo;
}

const EmptyIcon = styled.div`
  width: 1rem;
  height: 1rem;
`;

function renderToggleIcon(folder: FolderInfo, state: boolean): JSX.Element | null {
  if (folder.empty) {
    return <EmptyIcon></EmptyIcon>;
  } else if (state) {
    return <IconArrowDown width="1rem" height="1rem"></IconArrowDown>;
  } else {
    return <IconArrowRight width="1rem" height="1rem"></IconArrowRight>;
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
        <Group noWrap>
          {renderToggleIcon(props.folder, opened)}
          {renderFolderIcon(props.folder, opened)}
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
