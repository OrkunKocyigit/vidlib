import React, { useCallback, useContext, useEffect, useState } from 'react';
import { type VideoFile } from '../../../entities/VideoFile';
import { Box, Flex, Group, ThemeIcon, UnstyledButton } from '@mantine/core';
import { IconFolderOpen, IconVideo } from '@tabler/icons-react';
import { type IVideoContext, VideoContext } from '../entities/VideoContext';
import useStyles, { type VideoFileViewVariants } from './VideoFileView.styles';
import { listen } from '@tauri-apps/api/event';
import { useContextMenu } from 'mantine-contextmenu';
import { useTranslation } from 'react-i18next';
import { OpenPath } from '../../../service/OpenPath';

export interface VideoFileViewProps extends React.ComponentPropsWithoutRef<'div'> {
  video: VideoFile;
}
function VideoFileView(props: VideoFileViewProps): JSX.Element {
  const videoContext = useContext<IVideoContext>(VideoContext);
  const getVariant = useCallback(
    (watched?: boolean): VideoFileViewVariants => {
      return watched === true ? 'watched' : undefined;
    },
    [props.video.watched]
  );
  const { t } = useTranslation();
  const showContextMenu = useContextMenu();

  useEffect(() => {
    const unlisten = listen(
      'update_watch',
      ({ payload }: { payload: { id: string; watched: boolean } }) => {
        if (payload.id === props.video.id) {
          props.video.watched = payload.watched;
          setVariant(getVariant(payload.watched));
        }
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
  const [variant, setVariant] = useState(getVariant(props.video.watched));
  const { classes } = useStyles({ variant });

  function updateVideo(): void {
    if (videoContext.setVideo != null) {
      videoContext?.setVideo(props.video);
    }
  }

  function openPath(): void {
    OpenPath(props.video.path, true).catch((reason) => {
      console.error(reason);
    });
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
            openPath();
          }
        }
      ])}>
      <UnstyledButton onClick={updateVideo} className={classes.control}>
        <Group position={'apart'} spacing={0}>
          <Flex align={'center'}>
            <ThemeIcon size={20} variant={'outline'}>
              <IconVideo width={'0.9rem'} height={'0.9rem'} className={classes.icon}></IconVideo>
            </ThemeIcon>
            <Box ml={'md'}>{props.video.displayName}</Box>
          </Flex>
        </Group>
      </UnstyledButton>
    </div>
  );
}

export default VideoFileView;
