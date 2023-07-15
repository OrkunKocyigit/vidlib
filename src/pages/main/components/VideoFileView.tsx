import React, { useContext } from 'react';
import { type VideoFile } from '../../../entities/VideoFile';
import { Group, UnstyledButton } from '@mantine/core';
import { IconVideo } from '@tabler/icons-react';
import { type IVideoContext, VideoContext } from '../entities/VideoContext';

interface Props {
  video: VideoFile;
}
function VideoFileView(props: Props): JSX.Element {
  const videoContext = useContext<IVideoContext>(VideoContext);

  function updateVideo(): void {
    if (videoContext.setVideo != null) {
      videoContext?.setVideo(props.video);
    }
  }

  return (
    <UnstyledButton onClick={updateVideo}>
      <Group>
        <IconVideo></IconVideo>
        {props.video.displayName}
      </Group>
    </UnstyledButton>
  );
}

export default VideoFileView;
