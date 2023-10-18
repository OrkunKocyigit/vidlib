import React, { useContext, useEffect } from 'react';
import { type VideoFile } from '../../../entities/VideoFile';
import { Box, Flex, Group, ThemeIcon, UnstyledButton } from '@mantine/core';
import { IconFolderOpen, IconVideo } from '@tabler/icons-react';
import { type IVideoContext, VideoContext } from '../entities/VideoContext';
import classes from './VideoFileView.module.pcss';
import { listen } from '@tauri-apps/api/event';
import { useContextMenu } from 'mantine-contextmenu';
import { useTranslation } from 'react-i18next';
import { OpenPath } from '../../../service/OpenPath';

export interface VideoFileViewProps extends React.ComponentPropsWithoutRef<'div'> {
  video: VideoFile;
}

function VideoFileView(props: VideoFileViewProps): JSX.Element {
  const videoContext = useContext<IVideoContext>(VideoContext);
  const { t } = useTranslation();
  const showContextMenu = useContextMenu();

  useEffect(() => {
    const unlisten = listen(
      `update_watch_${props.video.id}`,
      ({ payload }: { payload: { watched: boolean } }) => {
        props.video.watched = payload.watched;
      }
    );
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

  function updateVideo(): void {
    if (videoContext.setVideo != null) {
      videoContext?.setVideo(props.video);
    }
  }

  return (
    <div
      className={props.className}
      onContextMenu={showContextMenu([
        {
          key: 'open',
          icon: <IconFolderOpen size={16} color={'blue'}></IconFolderOpen>,
          title: t('open.path'),
          onClick: () => {
            OpenPath(props.video.path, true).catch((reason) => {
              console.error(reason);
            });
          }
        }
      ])}>
      <UnstyledButton
        onClick={updateVideo}
        className={
          props.video.watched && props.video.selected(videoContext.video?.id)
            ? classes.controlSelectedWatched
            : props.video.watched
            ? classes.controlWatched
            : props.video.selected(videoContext.video?.id)
            ? classes.controlSelected
            : classes.control
        }>
        <Group justify="space-between" gap={0}>
          <Flex align="center">
            <ThemeIcon size={20} variant="outline">
              <IconVideo width="0.9rem" height="0.9rem" className={classes.icon}></IconVideo>
            </ThemeIcon>
            <Box ml={'md'}>{props.video.displayName}</Box>
          </Flex>
        </Group>
      </UnstyledButton>
    </div>
  );
}

export default VideoFileView;
