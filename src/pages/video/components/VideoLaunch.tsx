import { type VideoFile } from '../../../entities/VideoFile';
import { ActionIcon, Group, Text } from '@mantine/core';
import { IconPlayerPlay } from '@tabler/icons-react';
import { OpenVideo } from '../../../service/OpenVideo';

export interface VideoLaunchProps {
  video: VideoFile;
}

function VideoLaunch(props: VideoLaunchProps): JSX.Element {
  function openVideo(video: VideoFile): void {
    OpenVideo(video).catch((reason) => {
      console.error(reason);
    });
  }

  return (
    <Group align={'center'}>
      <ActionIcon color={'blue'} onClick={openVideo.bind(null, props.video)}>
        <IconPlayerPlay></IconPlayerPlay>
      </ActionIcon>
      <Text>Play</Text>
    </Group>
  );
}

export default VideoLaunch;
