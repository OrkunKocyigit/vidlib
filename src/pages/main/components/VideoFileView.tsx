import React, { useCallback, useContext } from 'react';
import { type VideoFile } from '../../../entities/VideoFile';
import { Box, Flex, Group, ThemeIcon, UnstyledButton } from '@mantine/core';
import { IconVideo } from '@tabler/icons-react';
import { type IVideoContext, VideoContext } from '../entities/VideoContext';
import useStyles, { type VideoFileViewVariants } from './VideoFileView.styles';

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
  const { classes } = useStyles({ variant: getVariant(props.video.watched) });

  function updateVideo(): void {
    if (videoContext.setVideo != null) {
      videoContext?.setVideo(props.video);
    }
  }

  return (
    <div className={props.className}>
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
