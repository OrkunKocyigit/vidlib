import { type FolderInfo } from '../../../entities/FolderInfo';
import React from 'react';
import FileTreeView from './FileTreeView';
import VideoFileView from './VideoFileView';
import { Box, Collapse, Flex, Group, ThemeIcon, UnstyledButton } from '@mantine/core';
import { IconArrowDown, IconArrowRight, IconFolder, IconFolderOpen } from '@tabler/icons-react';
import { useDisclosure } from '@mantine/hooks';
import styled from 'styled-components';
import useStyles from './FolderInfoView.styles';

interface FolderInfoProps {
  folder: FolderInfo;
}

const EmptyIcon = styled.div`
  width: 1rem;
  height: 1rem;
`;

function renderToggleIcon(
  folder: FolderInfo,
  state: boolean,
  cssClass: string
): JSX.Element | null {
  if (folder.empty) {
    return <EmptyIcon className={cssClass}></EmptyIcon>;
  } else if (state) {
    return <IconArrowDown width="1rem" height="1rem" className={cssClass}></IconArrowDown>;
  } else {
    return <IconArrowRight width="1rem" height="1rem" className={cssClass}></IconArrowRight>;
  }
}

function renderFolderIcon(folder: FolderInfo, state: boolean): JSX.Element | null {
  if (folder.empty || !state) {
    return <IconFolder width="0.9rem"></IconFolder>;
  } else {
    return <IconFolderOpen width="0.9rem"></IconFolderOpen>;
  }
}

function FolderInfoView(props: FolderInfoProps): JSX.Element | null {
  const [opened, { toggle }] = useDisclosure(false);
  const { classes } = useStyles();
  return (
    <>
      <UnstyledButton onClick={toggle} className={classes.control}>
        <Group position={'apart'} spacing={0} noWrap>
          <Flex align={'center'} mr={'md'}>
            <ThemeIcon size={20} variant={'outline'}>
              {renderFolderIcon(props.folder, opened)}
            </ThemeIcon>
            <Box ml={'md'}>{props.folder.displayName}</Box>
          </Flex>
          {renderToggleIcon(props.folder, opened, classes.icon)}
        </Group>
      </UnstyledButton>
      {!props.folder.empty ? (
        <Collapse in={opened}>
          <FileTreeView folders={props.folder.folders} className={classes.link}></FileTreeView>
          {props.folder.videos.map((video) => (
            <VideoFileView video={video} key={video.path} className={classes.link} />
          ))}
        </Collapse>
      ) : null}
    </>
  );
}

export default FolderInfoView;
