import { type FolderInfo } from '../../../entities/FolderInfo';
import React from 'react';
import FileTreeView from './FileTreeView';
import VideoFileView from './VideoFileView';
import {
  Box,
  Collapse,
  Flex,
  Group,
  Menu,
  ThemeIcon,
  UnstyledButton,
  useComponentDefaultProps
} from '@mantine/core';
import {
  IconArrowDown,
  IconArrowRight,
  IconFolder,
  IconFolderOpen,
  IconX
} from '@tabler/icons-react';
import { useDisclosure } from '@mantine/hooks';
import styled from 'styled-components';
import useStyles from './FolderInfoView.styles';
import { useContextMenu } from 'mantine-contextmenu';
import { useTranslation } from 'react-i18next';

export interface FolderInfoProps {
  folder: FolderInfo;
  showDelete?: boolean;
}

const defaultProps: Partial<FolderInfoProps> = {
  showDelete: false
};

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
  const { folder, showDelete } = useComponentDefaultProps('FolderInfo', defaultProps, props);
  const [opened, { toggle }] = useDisclosure(false);
  const { classes } = useStyles();
  const showContextMenu = useContextMenu();
  const { t } = useTranslation();

  function deletePath(): void {
    console.info(props.folder.path);
  }

  return (
    <>
      <UnstyledButton
        onClick={toggle}
        className={classes.control}
        onContextMenu={
          showDelete === true
            ? showContextMenu([
                {
                  key: 'delete',
                  icon: <IconX size={16} color={'red'}></IconX>,
                  title: t('video.delete'),
                  onClick: () => {
                    deletePath();
                  }
                }
              ])
            : undefined
        }>
        <Group spacing={0} noWrap>
          <Flex align={'center'} mr={'auto'} pr={'md'}>
            <ThemeIcon size={20} variant={'outline'}>
              {renderFolderIcon(folder, opened)}
            </ThemeIcon>
            <Box ml={'md'}>{folder.displayName}</Box>
          </Flex>
          {renderToggleIcon(folder, opened, classes.icon)}
        </Group>
      </UnstyledButton>
      {!folder.empty ? (
        <Collapse in={opened}>
          <FileTreeView folders={folder.folders} className={classes.link}></FileTreeView>
          {folder.videos.map((video) => (
            <VideoFileView
              video={video}
              key={`${video.path}${video.depth}`}
              className={classes.link}
            />
          ))}
        </Collapse>
      ) : null}
      <Menu shadow={'md'} width={200}></Menu>
    </>
  );
}

export default FolderInfoView;
