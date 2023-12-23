import { type FolderInfo } from '../../../entities/FolderInfo';
import React, { useCallback, useEffect, useState } from 'react';
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
  useProps
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
import { type ContextMenuItemOptions, useContextMenu } from 'mantine-contextmenu';
import { useTranslation } from 'react-i18next';
import { DeletePath } from '../../../service/DeletePath';
import { OpenPath } from '../../../service/OpenPath';
import { listen } from '@tauri-apps/api/event';
import classes from './FolderInfoView.module.pcss';

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
  const { folder, showDelete } = useProps('FolderInfo', defaultProps, props);
  const [opened, { toggle }] = useDisclosure(false);
  const { showContextMenu } = useContextMenu();
  const { t } = useTranslation();
  const [watched, setWatched] = useState(folder.watched);

  function getDefaultMenu(): ContextMenuItemOptions[] {
    return [
      {
        key: 'open',
        icon: <IconFolderOpen size={16} color={'blue'}></IconFolderOpen>,
        title: t('open.path'),
        onClick: () => {
          OpenPath(folder.path).catch((reason) => {
            console.error(reason);
          });
        }
      }
    ];
  }

  const contextMenu = useCallback(() => {
    const menuItems = getDefaultMenu();
    if (showDelete === true) {
      menuItems.push({
        key: 'delete',
        icon: <IconX size={16} color={'red'}></IconX>,
        title: t('video.delete'),
        onClick: () => {
          DeletePath(folder.path).catch((reason) => {
            console.error(reason);
          });
        }
      });
    }
    return menuItems;
  }, [showDelete]);

  useEffect(() => {
    const eventName = `update_watch_${props.folder.id}`;
    const unlisten = listen(eventName, () => {
      if (props.folder.videos.length <= 0) {
        setWatched(false);
      } else {
        setWatched(
          props.folder.videos
            .map((value) => value.watched)
            .reduce((previousValue, currentValue) => previousValue && currentValue, true)
        );
      }
    });
    return () => {
      unlisten
        .then((value) => {
          value();
        })
        .catch((reason) => {
          console.error(reason);
        });
    };
  }, []);

  return (
    <>
      <UnstyledButton
        onClick={toggle}
        className={watched ? classes.watchedControl : classes.control}
        onContextMenu={showContextMenu(contextMenu())}>
        <Group gap={0} wrap="nowrap">
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
